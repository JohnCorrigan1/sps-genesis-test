mod abi;
mod pb;
use hex_literal::hex;
use pb::address;
use prost::Message;
use substreams::pb::substreams::module::input::store;
use substreams::{key, prelude::*};
use substreams::{log, store::StoreSetProto, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

// Bored Ape Club Contract
const TRACKED_CONTRACT: [u8; 20] = hex!("bc4ca0eda7647a8ab7c2061c2e118a18a936f13d");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn map_is_contracts(
    blk: eth::Block,
) -> Result<Option<address::IsAccounts>, substreams::errors::Error> {
    let is_accounts: Vec<_> = blk
        .transaction_traces
        .iter()
        .map(|tx| {
            tx.calls.iter().map(|call| {
                call.account_creations.iter().map(|accountCreation| {
                    let is_contract;
                    if call.code_changes.len() > 0 {
                        is_contract = true;
                        substreams::log::info!("call is contract");
                    } else {
                        is_contract = false;
                        substreams::log::info!("call is not contract");
                    }

                    address::IsAccount {
                        id: Hex::encode(&accountCreation.account),
                        is_contract,
                    }
                })
            })
        })
        .flatten()
        .flatten()
        .collect();

    Ok(Some(address::IsAccounts { is_accounts }))
}

#[substreams::handlers::store]
fn store_is_contracts(addresses: address::IsAccounts, store: StoreSetInt64) {
    for address in addresses.is_accounts {
        let is_contract = if address.is_contract { 1 } else { 0 };
        store.set(0, address.id, &is_contract);
    }
}

#[substreams::handlers::map]
fn map_address_txs(
    blk: eth::Block,
) -> Result<Option<address::AddressTxs>, substreams::errors::Error> {
    let address_txs: Vec<_> = blk
        .transaction_traces
        .iter()
        .map(|tx| {
            tx.calls.iter().map(|call| address::AddressTx {
                address: Hex::encode(call.caller.clone()),
            })
        })
        .flatten()
        .collect();

    Ok(Some(address::AddressTxs { address_txs }))
}

#[substreams::handlers::store]
fn store_address_txs(address_txs: address::AddressTxs, store: StoreAddBigInt) {
    for address_tx in address_txs.address_txs {
        store.add(0, &address_tx.address, substreams::scalar::BigInt::from(1));
    }
}

// #[substreams::handlers::map]
// fn graph_out(
//     addresses: address::AddressTxs,
//     is_contract_store: StoreGetInt64,
//     txs_store: StoreGetBigInt,
// ) -> Result<Option<address::Addresses>, substreams::errors::Error> {
//     let addresses: Vec<_> = addresses
//         .address_txs
//         .iter()
//         .map(|address_tx| {
//             let is_contract;
//             if let Some(is_account) = is_contract_store.get_at(0, &address_tx.address) {
//                 is_contract = is_account == 1;
//             } else {
//                 is_contract = false;
//             };

//             let num_txs: u64;
//             if let Some(txs) = txs_store.get_at(0, &address_tx.address) {
//                 num_txs = txs.try_into().unwrap();
//             } else {
//                 num_txs = 1;
//             };
//             // let is_account = 1;

//             address::Address {
//                 id: address_tx.address.clone(),
//                 is_contract,
//                 num_txs,
//             }
//         })
//         .collect();

//     Ok(Some(address::Addresses { addresses }))
// }

#[substreams::handlers::map]
fn graph_out(
    addresses: address::AddressTxs,
    is_contract_store: StoreGetInt64,
    txs_store: StoreGetBigInt,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for address_tx in addresses.address_txs {
        let is_contract;
        if let Some(is_account) = is_contract_store.get_at(0, &address_tx.address) {
            is_contract = is_account == 1;
        } else {
            is_contract = false;
        };

        let num_txs: u64;
        if let Some(txs) = txs_store.get_at(0, &address_tx.address) {
            num_txs = txs.try_into().unwrap();
        } else {
            num_txs = 1;
        };

        tables
            .update_row("Address", address_tx.address)
            .set("isContract", is_contract)
            .set("numTxs", num_txs);
    }

    Ok(tables.to_entity_changes())
}
