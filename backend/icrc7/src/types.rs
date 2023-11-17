use b3_utils::{ledger::ICRCAccount, Subaccount};
use candid::CandidType;
use serde_derive::Deserialize;

#[derive(CandidType)]
pub struct CollectionMetadata {
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_royalties: Option<u16>,
    pub icrc7_royalty_recipient: Option<ICRCAccount>,
    pub icrc7_description: Option<String>,
    pub icrc7_image: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
}

#[derive(CandidType)]
pub struct Standard {
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArgs {
    pub spender_subaccount: Option<Subaccount>,
    pub from: ICRCAccount,
    pub to: ICRCAccount,
    pub token_ids: Vec<u128>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub is_atomic: Option<bool>,
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs {
    pub from_subaccount: Option<Subaccount>,
    pub spender: ICRCAccount,
    pub token_ids: Option<Vec<u128>>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct MintArgs {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub to: ICRCAccount,
}
