mod jupiter_aggregator_program;
mod pb;
mod types;
mod utils;
mod zebec_card_program;
mod constants;


use borsh::BorshDeserialize;
use constants::{USDC_PUBKEY, WSOL_PUBKEY};
use pb::silver_card::v1::{CardBotDirectCardPurchase, CardPurchase, Deposit, DepositType, DirectCardPurchase, GenerateYield, InitCardBotUserAccount, Output, Withdraw, WithdrawYield};
use substreams::log_debug;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use types::Discriminator;
use utils::bytes_to_base58;
use zebec_card_program::instruction::TokenTypeV1;

#[substreams::handlers::map]
fn map_silver_card_data(mut blk: Block) -> Result<Output, substreams::errors::Error> {
    let timestamp = blk.block_time.as_ref().unwrap().timestamp;
    let block_height=  blk.block_height.as_ref().unwrap().block_height;
    let slot = blk.slot;
    let blockhash = blk.blockhash.clone();

    let mut output = Output::default();

    blk.transactions.retain(|tx| {
        let meta = match tx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };

        if meta.err.is_some() {
            return false;
        }

        if tx.transaction.is_none() {
            return false;
        }

        tx.resolved_accounts()
            .contains(&&zebec_card_program::id_bytes())
    });

    for confirmed_tx in blk.transactions_owned() {
        let cloned_confirmed_tx = confirmed_tx.clone();
        let accounts = cloned_confirmed_tx.resolved_accounts();
        
        if let Some(tx) = confirmed_tx.transaction {
            let tx_hash = tx.id();
            log_debug!("[map_silver_card_data] tx hash: {} ", &tx_hash);
            let message = tx.message.as_ref().unwrap();
            let meta: &substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta = confirmed_tx.meta.as_ref().unwrap();

            let swap_and_deposit_index =
                message.instructions.iter().enumerate().find_map(|(i, ix)| {
                    if accounts.get(ix.program_id_index as usize)
                        == Some(&&jupiter_aggregator_program::id_bytes())
                    {
                        Some(i as u32)
                    } else {
                        None
                    }
                });

            message.instructions.iter().filter(|&ix| {
                accounts.get(ix.program_id_index as usize) == Some(&&zebec_card_program::id_bytes())
            }).for_each(|ix| {
                let ix_accounts = ix
                .accounts
                .iter()
                .map(|i| {
                    // log_debug!("[map_silver_card_data] mapping ix index: {}", *i);
                    let account = accounts.get(*i as usize).expect(&format!(
                        "Index out of bound: Could not get account at index: {}",
                        *i as usize
                    ));
                    *account
                })
                .collect::<Vec<_>>();

                let discriminator: [u8; 8] = ix.data[..8].try_into().unwrap();

                match discriminator {
                    zebec_card_program::instruction::DepositV2::DISCRIMINATOR => {
                        let mut deposit = Deposit::default();
                        deposit.slot = slot;
                        deposit.block_height = block_height;
                        deposit.blockhash = blockhash.clone();
                        deposit.timestamp = timestamp;
                        deposit.tx_hash = tx_hash.clone(); 

                        let is_deposit_v1 = false;
                        if ix.data.len() ==  8 + 9 {
                            let is_deposit_v1 = true;
                            let data = zebec_card_program::instruction::DepositV1::deserialize(
                                &mut &ix.data[8..],
                            )
                            .expect("Could not deserialize Deposit instruction data");

                            deposit.input_amount = data.params.amount;
                            deposit.output_amount = data.params.amount;
                            deposit.input_token = bytes_to_base58(ix_accounts[2]);
                            deposit.output_token = bytes_to_base58(ix_accounts[2]);

                            match data.params.token_type {
                                TokenTypeV1::Native => deposit.set_deposit_type(DepositType::Native),
                                TokenTypeV1::Usdc => deposit.set_deposit_type(DepositType::Usdc),
                                _ => deposit.set_deposit_type(DepositType::NonNative),
                            }

                        } else {
                            let data = zebec_card_program::instruction::DepositV2::deserialize(
                                &mut &ix.data[8..],
                            )
                            .expect("Could not deserialize Deposit instruction data");
                            deposit.input_amount = data.params.amount;
                            deposit.output_amount = data.params.amount;
                            deposit.input_token = bytes_to_base58(&data.params.source_token_address);
                            deposit.output_token = bytes_to_base58(ix_accounts[2]);

                            let wsol = bytes_to_base58(&WSOL_PUBKEY);
                            let usdc = bytes_to_base58(&USDC_PUBKEY);

                            let deposit_type = if deposit.input_token == usdc { DepositType::Usdc }
                            else if deposit.input_token == wsol {DepositType::Native } 
                            else { DepositType::NonNative };
                            
                            deposit.set_deposit_type(DepositType::Native);
                        };

                        

                        deposit.depositor = bytes_to_base58(ix_accounts[0]);
                        deposit.user_vault = bytes_to_base58(ix_accounts[3]);
                        deposit.purchase_record = bytes_to_base58(ix_accounts[6]);

                        if swap_and_deposit_index.is_some() {
                            let jup_cpi_event_ixs = meta.inner_instructions.iter().filter_map(|inner_ix| {
                                if inner_ix.index == swap_and_deposit_index.unwrap() {
                                    Some(&inner_ix.instructions)
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .filter(|ix| {
                                if ix.data.len() >=  16 { 
                                    let ix_discriminator: [u8; 16] = ix.data[..16].try_into().unwrap();
                                    
                                    let discriminator: [u8; 16] = [
                                        constants::ANCHOR_EVENT_IX_TAG_LE, 
                                        jupiter_aggregator_program::events::SwapEvent::DISCRIMINATOR
                                    ].concat()
                                    .try_into()
                                    .unwrap();
                                    
                                    return ix_discriminator == discriminator;
                                }

                                false
                            })
                            .collect::<Vec<_>>();
                            
                            let jup_cpi_event_ixs_len = jup_cpi_event_ixs.len();
                            if jup_cpi_event_ixs_len >= 1 {
                                // there may be one of more cpi event ixs. the first ix includes swap event from input token to other token.
                                // the last ix includes swap event from other token to output token
                                
                                let first_event_data =
                                jupiter_aggregator_program::events::SwapEvent::deserialize(
                                    &mut &jup_cpi_event_ixs[0].data[16..]
                                )
                                .expect("Could not deserialize jup swap event");

                                deposit.input_token = bytes_to_base58(&first_event_data.input_mint);
                                deposit.input_amount = first_event_data.input_amount;
                            }                            
                        }

                        output.deposits.push(deposit);
                    },

                    zebec_card_program::instruction::Withdraw::DISCRIMINATOR => {
                        let mut withdraw = Withdraw::default();
                        withdraw.slot = slot;
                        withdraw.block_height = block_height;
                        withdraw.blockhash = blockhash.clone();
                        withdraw.timestamp = timestamp;
                        withdraw.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::Withdraw::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize Withdraw instruction data");

                        withdraw.amount = data.amount;
                        withdraw.token = bytes_to_base58(ix_accounts[2]);
                        withdraw.user_vault = bytes_to_base58(ix_accounts[3]);
                        withdraw.withdrawer = bytes_to_base58(ix_accounts[0]);

                        output.withdraws.push(withdraw);
                    },

                    zebec_card_program::instruction::BuyPrepaidDigitalCard::DISCRIMINATOR => {
                        let mut card_purchase = CardPurchase::default();
                        card_purchase.slot = slot;
                        card_purchase.block_height = block_height;
                        card_purchase.blockhash = blockhash.clone();
                        card_purchase.timestamp = timestamp;
                        card_purchase.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::BuyPrepaidDigitalCard::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize BuyPrepaidDigitalCard instruction data");

                        card_purchase.card_id = data.params.index;
                        card_purchase.card_type = data.params.card_type;
                        card_purchase.amount = data.params.amount;
                        card_purchase.buyer = bytes_to_base58(ix_accounts[0]);
                        card_purchase.buyer_vault = bytes_to_base58(ix_accounts[2]);
                        card_purchase.purchase_record = bytes_to_base58(ix_accounts[12]);

                        output.card_purchases.push(card_purchase);
                    },

                    zebec_card_program::instruction::GenerateYield::DISCRIMINATOR => {
                        let mut generate_yield = GenerateYield::default();
                        generate_yield.slot = slot;
                        generate_yield.block_height = block_height;
                        generate_yield.blockhash = blockhash.clone();
                        generate_yield.timestamp = timestamp;
                        generate_yield.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::GenerateYield::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize GenerateYield instruction data");

                        generate_yield.amount = data.amount;
                        generate_yield.user = bytes_to_base58(ix_accounts[0]);
                        generate_yield.user_vault = bytes_to_base58(ix_accounts[1]);

                        output.generate_yields.push(generate_yield);
                    },

                    zebec_card_program::instruction::WithdrawYield::DISCRIMINATOR => {
                        let mut withdraw_yield = WithdrawYield::default();
                        withdraw_yield.slot = slot;
                        withdraw_yield.block_height = block_height;
                        withdraw_yield.blockhash = blockhash.clone();
                        withdraw_yield.timestamp = timestamp;
                        withdraw_yield.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::WithdrawYield::deserialize(&mut &ix.data[8..])
                        .expect("Could not deserialize WithdrawYield instruction data");
                    
                        withdraw_yield.amount = data.amount;
                        withdraw_yield.withdraw_all= data.withdraw_all;
                        withdraw_yield.user = bytes_to_base58(ix_accounts[0]);
                        withdraw_yield.user_vault = bytes_to_base58(ix_accounts[1]);

                        output.withdraw_yields.push(withdraw_yield);
                    },

                    zebec_card_program::instruction::BuyCardDirect::DISCRIMINATOR => {
                        let mut direct_card_purchase = DirectCardPurchase::default();
                        direct_card_purchase.slot = slot;
                        direct_card_purchase.block_height = block_height;
                        direct_card_purchase.blockhash = blockhash.clone();
                        direct_card_purchase.timestamp = timestamp;
                        direct_card_purchase.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::BuyCardDirect::deserialize(&mut &ix.data[8..])
                            .expect("Could not deserialize BuyCardDirect instruction data");

                        direct_card_purchase.input_token = bytes_to_base58(&data.params.source_token_address);
                        direct_card_purchase.output_token = bytes_to_base58(ix_accounts[1]);
                        direct_card_purchase.input_amount = data.params.amount;
                        direct_card_purchase.output_amount = data.params.amount;
                        direct_card_purchase.card_id = data.params.index;
                        direct_card_purchase.card_type = data.params.card_type;
                        direct_card_purchase.buyer = bytes_to_base58(ix_accounts[0]);
                        direct_card_purchase.buyer_purchase = bytes_to_base58(ix_accounts[3]);
                        direct_card_purchase.purchase_record = bytes_to_base58(ix_accounts[9]);

                        if swap_and_deposit_index.is_some() {
                            let jup_cpi_event_ixs = meta.inner_instructions.iter().filter_map(|inner_ix| {
                                if inner_ix.index == swap_and_deposit_index.unwrap() {
                                    Some(&inner_ix.instructions)
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .filter(|ix| {
                                if ix.data.len() >=  16 { 
                                    let ix_discriminator: [u8; 16] = ix.data[..16].try_into().unwrap();
                                    
                                    let discriminator: [u8; 16] = [
                                        constants::ANCHOR_EVENT_IX_TAG_LE, 
                                        jupiter_aggregator_program::events::SwapEvent::DISCRIMINATOR
                                    ].concat()
                                    .try_into()
                                    .unwrap();
                                    
                                    return ix_discriminator == discriminator;
                                }

                                false
                            })
                            .collect::<Vec<_>>();
                            
                            let jup_cpi_event_ixs_len = jup_cpi_event_ixs.len();
                            if jup_cpi_event_ixs_len >= 1 {
                                // there may be one of more cpi event ixs. the first ix includes swap event from input token to other token.
                                // the last ix includes swap event from other token to output token
                                
                                let first_event_data =
                                jupiter_aggregator_program::events::SwapEvent::deserialize(
                                    &mut &jup_cpi_event_ixs[0].data[16..]
                                )
                                .expect("Could not deserialize jup swap event");

                                direct_card_purchase.input_token = bytes_to_base58(&first_event_data.input_mint);
                                direct_card_purchase.input_amount = first_event_data.input_amount;
                            }                            
                        }

                        output.direct_card_purhcases.push(direct_card_purchase);
                    },

                    zebec_card_program::instruction::InitBotPda::DISCRIMINATOR => {
                        let mut init_card_bot_user_account = InitCardBotUserAccount::default();
                        init_card_bot_user_account.slot = slot;
                        init_card_bot_user_account.block_height = block_height;
                        init_card_bot_user_account.blockhash = blockhash.clone();
                        init_card_bot_user_account.timestamp = timestamp;
                        init_card_bot_user_account.tx_hash = tx_hash.clone();

                        let data = zebec_card_program::instruction::InitBotPda::deserialize(&mut &ix.data[8..])
                            .expect("Could not deserialize InitBotPda instruction data");

                        init_card_bot_user_account.user_id = data.params.user_id;
                        init_card_bot_user_account.user_custody = bytes_to_base58(ix_accounts[1]);
                        init_card_bot_user_account.usdc_token = bytes_to_base58(ix_accounts[4]);
                        init_card_bot_user_account.admin = bytes_to_base58(ix_accounts[0]);

                        output.card_bot_user_account_inits.push(init_card_bot_user_account);
                    },

                    zebec_card_program::instruction::BuyCardBot::DISCRIMINATOR => {
                        let mut card_bot_direct_card_purchase = CardBotDirectCardPurchase::default();
                        card_bot_direct_card_purchase.slot = slot;
                        card_bot_direct_card_purchase.block_height = block_height;
                        card_bot_direct_card_purchase.blockhash = blockhash.clone();
                        card_bot_direct_card_purchase.timestamp = timestamp;
                        card_bot_direct_card_purchase.tx_hash = tx_hash.clone();
                        
                        let data = zebec_card_program::instruction::BuyCardBot::deserialize(&mut &ix.data[8..])
                            .expect("Could not deserialize BuyCardBot instruction data");

                        card_bot_direct_card_purchase.user_id = data.params.user_id;
                        card_bot_direct_card_purchase.input_token = bytes_to_base58(&data.params.source_token_address);
                        card_bot_direct_card_purchase.output_token = bytes_to_base58(ix_accounts[1]);
                        card_bot_direct_card_purchase.input_amount = data.params.amount;
                        card_bot_direct_card_purchase.output_amount = data.params.amount;
                        card_bot_direct_card_purchase.user_custody = bytes_to_base58(ix_accounts[2]);
                        card_bot_direct_card_purchase.admin = bytes_to_base58(ix_accounts[0]); 

                        if swap_and_deposit_index.is_some() {
                            let jup_cpi_event_ixs = meta.inner_instructions.iter().filter_map(|inner_ix| {
                                if inner_ix.index == swap_and_deposit_index.unwrap() {
                                    Some(&inner_ix.instructions)
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .filter(|ix| {
                                if ix.data.len() >=  16 { 
                                    let ix_discriminator: [u8; 16] = ix.data[..16].try_into().unwrap();
                                    
                                    let discriminator: [u8; 16] = [
                                        constants::ANCHOR_EVENT_IX_TAG_LE, 
                                        jupiter_aggregator_program::events::SwapEvent::DISCRIMINATOR
                                    ].concat()
                                    .try_into()
                                    .unwrap();
                                    
                                    return ix_discriminator == discriminator;
                                }

                                false
                            })
                            .collect::<Vec<_>>();
                            
                            let jup_cpi_event_ixs_len = jup_cpi_event_ixs.len();
                            if jup_cpi_event_ixs_len >= 1 {
                                // there may be one of more cpi event ixs. the first ix includes swap event from input token to other token.
                                // the last ix includes swap event from other token to output token
                                
                                let first_event_data =
                                jupiter_aggregator_program::events::SwapEvent::deserialize(
                                    &mut &jup_cpi_event_ixs[0].data[16..]
                                )
                                .expect("Could not deserialize jup swap event");

                                card_bot_direct_card_purchase.input_token = bytes_to_base58(&first_event_data.input_mint);
                                card_bot_direct_card_purchase.input_amount = first_event_data.input_amount;
                            }                            
                        }

                        output.card_bot_direct_card_purchases.push(card_bot_direct_card_purchase);                       
                    },
                    
                    _ => (),
                };
            });
        }
    }

    Ok(output)
}

// pub fn process_deposit_instruction(
//     ix: &CompiledInstruction,
//     meta: &TransactionStatusMeta,
//     accounts: &[Vec<u8>],
//     swap_and_deposit_ix_index: Option<usize>,
//     tx_hash: &str,
//     timestamp: i64,
// ) -> Deposit {
//     unimplemented!()
// }
