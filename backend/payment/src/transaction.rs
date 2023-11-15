use candid::CandidType;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(CandidType, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub jsonrpc: String,
    pub id: i64,
    pub result: Result,
}

#[derive(CandidType, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub block_hash: String,
    pub block_number: String,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transaction_index: String,
    pub value: String,
    pub v: String,
    pub r: String,
    pub s: String,
}
