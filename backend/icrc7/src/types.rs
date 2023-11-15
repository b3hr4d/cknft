#![allow(non_snake_case)]

use candid::{CandidType, Principal};
use icrc_ledger_types::icrc1::account::{Account, Subaccount};
use serde_derive::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct InitArg{
    pub tx_window: u16, // input should be in hours format
    pub permitted_drift: u16, // input should be in minutes format
    pub name: String,
    pub symbol: String,
    pub minting_authority: Option<Principal>,
    pub royalties: Option<u16>,
    pub royalties_recipient: Option<Account>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub supply_cap: Option<u128>,
}

#[derive(CandidType)]
pub struct CollectionMetadata {
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<Account>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
}

#[derive(CandidType)]
pub struct Standard{
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs{
    pub spender_subaccount: Option<Subaccount>,
    pub from: Account,
    pub to: Account,
    pub token_ids: Vec<u128>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub is_atomic: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs{
    pub from_subaccount: Option<Subaccount>,
    pub spender: Account,
    pub token_ids: Option<Vec<u128>>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct MintArgs{
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub to: Account,
}