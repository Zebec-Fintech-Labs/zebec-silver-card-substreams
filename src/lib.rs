mod jupiter_aggregator_program;
mod pb;
mod types;
mod utils;
mod zebec_card_program;

use crate::{
    jupiter_aggregator_program::JupSwapEvent,
    pb::silver_card::v1::{Deposit, DepositType},
};
use borsh::BorshDeserialize;
use pb::silver_card::v1::Output;
use substreams::log_debug;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use types::Discriminator;
use utils::bytes_to_base58;

#[substreams::handlers::map]
fn map_silver_card_data(blk: Block) -> Result<Output, substreams::errors::Error> {
    let timestamp = blk.block_time.as_ref().unwrap().timestamp;
    let mut output = Output::default();

    blk.transactions_owned().filter(|trx| {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };
        
        if meta.err.is_some() {
            return false;
        }

        if trx.transaction.is_none() {
             return false;
        }
        
        trx.resolved_accounts().contains(&&zebec_card_program::id_bytes())
    }).for_each(|confirmed_tx| {
        let accounts = &confirmed_tx.resolved_accounts();

        if let Some(trx) = &confirmed_tx.transaction {
            let tx_hash = bs58::encode(&trx.signatures[0]).into_string();
            log_debug!("[map_silver_card_data] tx hash: {}\n", tx_hash);

            let message = trx.message.as_ref().unwrap();
            let meta = confirmed_tx.meta.as_ref().unwrap();


            // let is_card_trx = message
            //     .instructions
            //     .iter()
            //     .any(|ix| accounts[ix.program_id_index as usize] == zebec_card_program::id_bytes());
            // assert!(is_card_trx);
            assert!(
                !meta.inner_instructions_none
                    && meta.inner_instructions.len() != message.instructions.len(),
            );

            let swap_and_deposit_ix = message.instructions.iter().enumerate().find(|ix| {
                accounts.get(ix.1.program_id_index as usize) == Some(&&jupiter_aggregator_program::id_bytes())
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
                        "Index out of bound: mapping ix index: {}",
                        *i as usize
                    ));
                    *account
                })
                .collect::<Vec<_>>();

                let discriminator: [u8; 8] = ix.data[..8].try_into().unwrap();

                match discriminator {
                    zebec_card_program::instruction::Deposit::DISCRIMINATOR => {
                        let mut deposit = Deposit::default();
                        deposit.tx_hash = tx_hash.to_string();
                        deposit.timestamp = timestamp;

                        let data = zebec_card_program::instruction::Deposit::deserialize(
                            &mut &ix.data[8..],
                        )
                        .expect("Could not deserialize deposit instruction data");

                        let deposit_type = match data._params.token_type {
                            zebec_card_program::TokenType::Native => DepositType::Native,
                            zebec_card_program::TokenType::NonNative => DepositType::NonNative,
                            zebec_card_program::TokenType::Usdc => DepositType::Usdc,
                        };

                        deposit.set_deposit_type(deposit_type);
                        deposit.source = bytes_to_base58(ix_accounts[0]);
                        deposit.destination = bytes_to_base58(ix_accounts[3]);
                        deposit.output_token = bytes_to_base58(ix_accounts[2]);
                        deposit.output_amount = data._params.amount;
                        deposit.input_token = bytes_to_base58(ix_accounts[2]);
                        deposit.input_amount = data._params.amount;

                        if swap_and_deposit_ix.is_some() {
                            let index = swap_and_deposit_ix.unwrap().0;
                            substreams::log::println(format!("[map_silver_card_data] swap cpi event ix index: {}; inner ix len: {}", index, meta.inner_instructions.len()));

                            let jup_cpi_event_ix = meta.inner_instructions[index]
                                .instructions
                                .iter()
                                .find(|ix| {
                                    accounts.get(ix.program_id_index as usize)
                                        == Some(&&jupiter_aggregator_program::id_bytes())
                                })
                                .expect("Could not find jupiter cpi event instruction");

                            let event_data =
                                JupSwapEvent::deserialize(&mut &jup_cpi_event_ix.data[16..])
                                    .expect("Could not deserialize jup swap event");

                            deposit.input_token = bytes_to_base58(&event_data.input_mint);
                            deposit.input_amount = event_data.input_amount;
                            deposit.output_token = bytes_to_base58(&event_data.output_mint);
                            deposit.output_amount = event_data.output_amount;
                        }

                        output.deposits.push(deposit);
                    }
                    _ => (),
                };
            });
        }
    });

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
