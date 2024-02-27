mod abi;
mod pb;
use pb::address;
use substreams::prelude::*;
use substreams::{store::StoreSetProto, Hex};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::Call;
use substreams_ethereum::pb::sf::ethereum::r#type::v2 as eth;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_is_contracts(
    blk: eth::Block,
) -> Result<Option<address::IsAccounts>, substreams::errors::Error> {
    let is_accounts: Vec<_> = blk
        .transaction_traces
        .iter()
        .map(|tx| {
            tx.calls.iter().map(|call| {
                // substreams::log::info!("account creations: {:?}", call.account_creations);
                // substreams::log::info!("code changes: {:?}", call.code_changes.len() > 0);
                // substreams::log::info!("call {:?}", call);
                // substreams::log::info!("call type {:?}", call.call_type);
                call.account_creations.iter().map(|account_creation| {
                    let is_contract;
                    if call.code_changes.len() > 0 {
                        is_contract = true;
                    } else {
                        is_contract = false;
                    }
                    // if call.account_creations.len() > 1 && call.code_changes.len() == 0 {
                    // panic!("multiple account creations");
                    address::IsAccount {
                        id: Hex::encode(&account_creation.account),
                        is_contract,
                    }
                    // } else {
                    // address::IsAccount {
                    //     id: "1".to_string(),
                    //     is_contract: false,
                    // }
                    // }
                })
            })
        })
        .flatten()
        .flatten()
        .collect();

    Ok(Some(address::IsAccounts { is_accounts }))
}

// fn get_is_account(call: &Call) -> Result<address::IsAccount, substreams::errors::Error> {
//     Ok(call
//         .account_creations
//         .iter()
//         .map(|account_creation| {
//             let is_contract;
//             if call.code_changes.len() > 0 {
//                 is_contract = true;
//             } else {
//                 is_contract = false;
//             }

//             address::IsAccount {
//                 id: Hex::encode(&account_creation.account),
//                 is_contract,
//             }
//         })
//         .collect())
// }

#[substreams::handlers::store]
fn store_is_contracts(addresses: address::IsAccounts, store: StoreSetProto<address::IsContract>) {
    for address in addresses.is_accounts {
        store.set(
            0,
            &address.id,
            &address::IsContract {
                is_contract: address.is_contract,
            },
        );
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
                address: Hex::encode(&call.caller),
            })
        })
        .flatten()
        .collect();

    Ok(Some(address::AddressTxs { address_txs }))
}

#[substreams::handlers::store]
fn store_address_txs(address_txs: address::AddressTxs, store: StoreAddInt64) {
    for address_tx in address_txs.address_txs {
        store.add(0, address_tx.address, 1);
    }
}

#[substreams::handlers::map]
fn graph_out(
    addresses: address::AddressTxs,
    is_contract_store: StoreGetProto<address::IsContract>,
    txs_store: StoreGetInt64,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for address_tx in addresses.address_txs {
        let is_contract;
        if let Some(is_account) = is_contract_store.get_at(0, &address_tx.address) {
            is_contract = is_account.is_contract;
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
            .update_row("Address", &address_tx.address)
            .set("isContract", is_contract)
            .set("numTxs", num_txs);
    }

    Ok(tables.to_entity_changes())
}
