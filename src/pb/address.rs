// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressesAndTxs {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_contract: bool,
    #[prost(uint64, tag="3")]
    pub num_txs: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumTxs {
    #[prost(uint32, tag="1")]
    pub num_txs: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsContract {
    #[prost(bool, tag="1")]
    pub is_contract: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsAccounts {
    #[prost(message, repeated, tag="1")]
    pub is_accounts: ::prost::alloc::vec::Vec<IsAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsAccount {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_contract: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressTxs {
    #[prost(message, repeated, tag="1")]
    pub address_txs: ::prost::alloc::vec::Vec<AddressTx>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressTx {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Addresses {
    #[prost(message, repeated, tag="1")]
    pub addresses: ::prost::alloc::vec::Vec<Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_contract: bool,
    #[prost(uint64, tag="3")]
    pub num_txs: u64,
}
// @@protoc_insertion_point(module)
