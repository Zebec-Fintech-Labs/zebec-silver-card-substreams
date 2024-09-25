mod jupiter_aggregator_program;
mod pb;
mod types;
mod utils;
mod zebec_card_program;
mod constants;


use crate::pb::silver_card::v1::{Deposit, DepositType};
use borsh::BorshDeserialize;
use pb::silver_card::v1::{CardPurchase, GenerateYield, Output, Withdraw, WithdrawYield};
use substreams::log_debug;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use types::Discriminator;
use utils::bytes_to_base58;

#[substreams::handlers::map]
fn map_silver_card_data(mut blk: Block) -> Result<Output, substreams::errors::Error> {
    let timestamp = blk.block_time.as_ref().unwrap().timestamp;
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
            let meta = confirmed_tx.meta.as_ref().unwrap();

            let swap_and_deposit_index =
                message.instructions.iter().enumerate().find_map(|(i, ix)| {
                    if accounts.get(ix.program_id_index as usize)
                        == Some(&&jupiter_aggregator_program::id().to_vec())
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
                    zebec_card_program::instruction::Deposit::DISCRIMINATOR => {
                        let mut deposit = Deposit::default();
                        deposit.tx_hash = tx_hash.clone();
                        deposit.timestamp = timestamp;

                        let data = zebec_card_program::instruction::Deposit::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize Deposit instruction data");

                        let deposit_type = match data._params.token_type {
                            zebec_card_program::TokenType::Native => DepositType::Native,
                            zebec_card_program::TokenType::NonNative => DepositType::NonNative,
                            zebec_card_program::TokenType::Usdc => DepositType::Usdc,
                        };

                        deposit.set_deposit_type(deposit_type);
                        deposit.depositor = bytes_to_base58(ix_accounts[0]);
                        deposit.user_vault = bytes_to_base58(ix_accounts[3]);
                        deposit.output_token = bytes_to_base58(ix_accounts[2]);
                        deposit.output_amount = data._params.amount;
                        deposit.input_token = bytes_to_base58(ix_accounts[2]);
                        deposit.input_amount = data._params.amount;

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
                            if jup_cpi_event_ixs_len > 0 {
                                // there may be one of more cpi event ixs. the first ix includes swap event from input token to other token.
                                // the last ix includes swap event from other token to output token
                                
                                let first_event_data =
                                jupiter_aggregator_program::events::SwapEvent::deserialize(
                                    &mut &jup_cpi_event_ixs[0].data[16..]
                                )
                                .expect("Could not deserialize jup swap event");

                                deposit.input_token = bytes_to_base58(&first_event_data.input_mint);
                                deposit.input_amount = first_event_data.input_amount;

                                let last_event_data = if jup_cpi_event_ixs_len == 1 {
                                    first_event_data
                                } else {
                                    jupiter_aggregator_program::events::SwapEvent::deserialize(
                                        &mut &jup_cpi_event_ixs[jup_cpi_event_ixs_len - 1].data[16..]
                                    )
                                    .expect("Could not deserialize jup swap event")
                                };

                                deposit.output_token = bytes_to_base58(&last_event_data.output_mint);
                                deposit.output_amount = last_event_data.output_amount;
                            }

                            
                        }

                        output.deposits.push(deposit);
                    },

                    zebec_card_program::instruction::Withdraw::DISCRIMINATOR => {
                        let mut withdraw = Withdraw::default();

                        let data = zebec_card_program::instruction::Withdraw::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize Withdraw instruction data");

                        withdraw.tx_hash = tx_hash.clone();
                        withdraw.user_vault = bytes_to_base58(ix_accounts[3]);
                        withdraw.withdrawer = bytes_to_base58(ix_accounts[0]);
                        withdraw.token = bytes_to_base58(ix_accounts[2]);
                        withdraw.amount = data._amount;
                        withdraw.timestamp = timestamp;

                        output.withdraws.push(withdraw);
                    },

                    zebec_card_program::instruction::BuyPrepaidDigitalCard::DISCRIMINATOR => {
                        let mut card_purchase = CardPurchase::default();

                        let data = zebec_card_program::instruction::BuyPrepaidDigitalCard::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize BuyPrepaidDigitalCard instruction data");

                        card_purchase.tx_hash = tx_hash.clone();
                        card_purchase.card_id = data._params.index;
                        card_purchase.buyer = bytes_to_base58(ix_accounts[0]);
                        card_purchase.buyer_vault = bytes_to_base58(ix_accounts[2]);
                        card_purchase.amount = data._params.amount;
                        card_purchase.card_type = data._params.card_type;
                        card_purchase.timestamp = timestamp;

                        output.card_purchases.push(card_purchase);
                    },

                    zebec_card_program::instruction::GenerateYield::DISCRIMINATOR => {
                        let mut generate_yield = GenerateYield::default();

                        let data = zebec_card_program::instruction::GenerateYield::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize GenerateYield instruction data");

                        generate_yield.tx_hash = tx_hash.clone();
                        generate_yield.user = bytes_to_base58(ix_accounts[0]);
                        generate_yield.user_vault = bytes_to_base58(ix_accounts[1]);
                        generate_yield.amount = data._amount;
                        generate_yield.timestamp = timestamp;

                        output.generate_yields.push(generate_yield);
                    },

                    zebec_card_program::instruction::WithdrawYield::DISCRIMINATOR => {
                        let mut withdraw_yield = WithdrawYield::default();

                        let data = zebec_card_program::instruction::WithdrawYield::deserialize(&mut &ix.data[8..])
                        .expect("Could not deserialize WithdrawYield instruction data");
                    
                        withdraw_yield.tx_hash = tx_hash.clone();
                        withdraw_yield.user = bytes_to_base58(ix_accounts[0]);
                        withdraw_yield.user_vault = bytes_to_base58(ix_accounts[1]);
                        withdraw_yield.amount = data._amount;
                        withdraw_yield.withdraw_all= data._withdraw_all;
                        withdraw_yield.timestamp = timestamp;
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
