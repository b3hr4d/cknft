use b3_utils::{ledger::ICRCAccount, Subaccount};
use candid::CandidType;
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(CandidType)]
pub struct CollectionMetadata {
    pub icrc7_symbol: String,
    pub icrc7_name: String,
    pub icrc7_description: Option<String>,
    pub icrc7_logo: Option<String>,
    pub icrc7_total_supply: u128,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_max_query_batch_size: Option<u128>,
    pub icrc7_max_update_batch_size: Option<u128>,
    pub icrc7_default_take_value: Option<u128>,
    pub icrc7_max_take_value: Option<u128>,
    pub icrc7_max_memo_size: Option<u128>,
    pub icrc7_atomic_batch_transfers: Option<bool>,
    pub icrc7_tx_window: u64,
    pub icrc7_permitted_drift: u64,
}

#[derive(CandidType)]
pub struct Standard {
    pub name: String,
    pub url: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: ICRCAccount,
    pub token_id: u128,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(
    Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default,
)]
#[serde(transparent)]
pub struct Memo(pub ByteBuf);

impl From<u64> for Memo {
    fn from(num: u64) -> Self {
        Self(ByteBuf::from(num.to_be_bytes().to_vec()))
    }
}

impl From<ByteBuf> for Memo {
    fn from(b: ByteBuf) -> Self {
        Self(b)
    }
}

impl From<Vec<u8>> for Memo {
    fn from(v: Vec<u8>) -> Self {
        Self::from(ByteBuf::from(v))
    }
}

impl From<Memo> for ByteBuf {
    fn from(memo: Memo) -> Self {
        memo.0
    }
}

#[derive(CandidType, Debug, Clone)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

#[derive(CandidType, Deserialize)]
pub struct ApprovalArgs {
    pub from_subaccount: Option<Subaccount>,
    pub spender: ICRCAccount,
    pub token_ids: Option<Vec<u128>>,
    pub expires_at: Option<u64>,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Debug, Clone)]
pub enum ApprovalError {
    Unauthorized { tokens_ids: Vec<u128> },
    TooOld,
    TemporaryUnavailable,
    GenericError { error_code: u128, msg: String },
}

#[derive(CandidType, Deserialize)]
pub struct MintArgs {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub to: ICRCAccount,
}

#[derive(Clone, CandidType, serde::Serialize, serde::Deserialize)]
pub struct MintStatus {
    pub id: u128,
    pub amount: u128,
    pub expiry: u64,
    pub state: MintState,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct SelfMintArgs {
    pub id: u128,
    pub to: String,
    pub msgid: u128,
    pub expiry: u64,
    pub signature: String,
}

#[derive(
    CandidType, serde::Serialize, serde::Deserialize, Default, Clone, Debug, PartialEq, Eq,
)]
pub enum MintState {
    #[default]
    Init,
    FundReceived,
    Signed,
    Confirmed,
    Expired,
}
