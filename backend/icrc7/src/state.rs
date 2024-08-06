use crate::{
    crypto::EcdsaSignature,
    types::{ApprovalError, TransferError},
    types::{CollectionMetadata, Memo, MintStatus},
};
use b3_utils::{
    ledger::{ICRC1MetadataValue, ICRCAccount},
    memory::{
        init_stable_mem_refcell,
        types::{Bound, DefaultStableBTreeMap, DefaultStableCell, DefaultStableVec, Storable},
    },
    nonce::Nonce,
    Subaccount,
};
use candid::{CandidType, Decode, Encode, Nat};
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    pub static CONFIG: RefCell<DefaultStableCell<CollectionConfig>> = init_stable_mem_refcell("config", 1).unwrap();
    pub static TOKENS: RefCell<DefaultStableBTreeMap<u128, Token>> = init_stable_mem_refcell("tokens", 2).unwrap();
    pub static TRANSFER_LOG: RefCell<DefaultStableVec<TransferLog>> = init_stable_mem_refcell("transfer_log", 3).unwrap();
    pub static TRANSACTION_ID: RefCell<DefaultStableCell<u128>> = init_stable_mem_refcell("transaction_id", 4).unwrap();
    pub static TOTAL_SUPPLY: RefCell<DefaultStableCell<u128>> = init_stable_mem_refcell("total_supply", 5).unwrap();
    pub static NONCE_MAP: RefCell<DefaultStableBTreeMap<Subaccount, Nonce>> = init_stable_mem_refcell("nonce_map", 6).unwrap();
    pub static STATUS_MAP: RefCell<DefaultStableBTreeMap<u128, MintStatus>> = init_stable_mem_refcell("status_map", 7).unwrap();
    pub static SIGNATURE_MAP: RefCell<DefaultStableBTreeMap<u128, EcdsaSignature>> = init_stable_mem_refcell("signature_map", 8).unwrap();
    pub static PUBLIC_KEY: RefCell<DefaultStableCell<Vec<u8>>> = init_stable_mem_refcell("cknft_state", 9).unwrap();
}

pub fn get_icrc7_config() -> CollectionConfig {
    CONFIG.with(|ckicp_config| {
        let ckicp_config = ckicp_config.borrow();
        ckicp_config.get().clone()
    })
}

#[derive(Default, CandidType, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub total_supply: u128,
    pub supply_cap: Option<u128>,
    pub max_query_batch_size: Option<u128>,
    pub max_update_batch_size: Option<u128>,
    pub default_take_value: Option<u128>,
    pub max_take_value: Option<u128>,
    pub max_memo_size: Option<u128>,
    pub atomic_batch_transfers: Option<bool>,
    pub tx_window: u64,
    pub permitted_drift: u64,
    pub cknft_eth_address: String,
    pub ecdsa_key_name: String,
}

impl Storable for CollectionConfig {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl CollectionConfig {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn logo(&self) -> Option<String> {
        self.logo.clone()
    }

    pub fn supply_cap(&self) -> Option<u128> {
        self.supply_cap.clone()
    }

    pub fn metadata(&self) -> CollectionMetadata {
        CollectionMetadata {
            icrc7_name: self.name.clone(),
            icrc7_symbol: self.symbol.clone(),
            icrc7_description: self.description.clone(),
            icrc7_logo: self.logo.clone(),
            icrc7_total_supply: self.total_supply,
            icrc7_supply_cap: self.supply_cap,
            icrc7_max_query_batch_size: self.max_query_batch_size,
            icrc7_max_update_batch_size: self.max_update_batch_size,
            icrc7_default_take_value: self.default_take_value,
            icrc7_max_take_value: self.max_take_value,
            icrc7_max_memo_size: self.max_memo_size,
            icrc7_atomic_batch_transfers: self.atomic_batch_transfers,
            icrc7_tx_window: self.tx_window,
            icrc7_permitted_drift: self.permitted_drift,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Token {
    pub id: u128,
    pub owner: ICRCAccount,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub description: Option<String>,
    pub approvals: Vec<Approval>,
}

impl Token {
    pub fn token_metadata(&self) -> Vec<(String, ICRC1MetadataValue)> {
        let mut metadata = Vec::new();
        metadata.push((
            "Id".to_string(),
            ICRC1MetadataValue::Nat(Nat::from(self.id)),
        ));
        metadata.push(("Name".into(), ICRC1MetadataValue::Text(self.name.clone())));
        if self.image.is_some() {
            let buf = ByteBuf::from(self.image.as_ref().unwrap().clone());
            metadata.push(("Image".into(), ICRC1MetadataValue::Blob(buf)))
        }
        if self.description.is_some() {
            let value = self.description.as_ref().unwrap().clone();
            metadata.push(("Description".into(), ICRC1MetadataValue::Text(value)))
        }
        metadata
    }

    pub fn owner(&self) -> ICRCAccount {
        self.owner.clone()
    }

    pub fn approval_check(&self, current_time: u64, account: &ICRCAccount) -> bool {
        for approval in self.approvals.iter() {
            if approval.account == *account {
                if approval.expires_at.is_none() {
                    return true;
                } else if approval.expires_at >= Some(current_time) {
                    return true;
                }
            }
        }
        false
    }

    pub fn approve(
        &mut self,
        caller: &ICRCAccount,
        approval: Approval,
    ) -> Result<(), ApprovalError> {
        if self.owner == approval.account {
            ic_cdk::trap("Self Approve")
        }
        if *caller != self.owner {
            return Err(ApprovalError::Unauthorized {
                tokens_ids: vec![self.id],
            });
        } else {
            self.approvals.push(approval);
            Ok(())
        }
    }

    pub fn transfer(
        &mut self,
        permitted_time: u64,
        caller: &ICRCAccount,
        to: ICRCAccount,
    ) -> Result<(), TransferError> {
        if self.owner == to {
            ic_cdk::trap("Self Transfer")
        }
        if self.owner != *caller && !self.approval_check(permitted_time, caller) {
            return Err(TransferError::Unauthorized);
        } else {
            self.owner = to;
            self.approvals.clear();
            return Ok(());
        }
    }
}

impl Storable for Token {
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

pub fn id_validity_check(id: u128) {
    TOKENS.with(|tokens| match tokens.borrow().get(&id) {
        Some(_) => (),
        None => {
            let error_msg = format!("Invalid Token Id: {:?}", id);
            ic_cdk::trap(&error_msg);
        }
    });
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TransferLog {
    pub id: u128,
    pub at: u64,
    pub memo: Option<Memo>,
    pub from: ICRCAccount,
    pub to: ICRCAccount,
}

impl Storable for TransferLog {
    const BOUND: Bound = Bound::Bounded {
        max_size: 200,
        is_fixed_size: false,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

pub fn increment_tx_id() -> u128 {
    TRANSACTION_ID.with(|id| {
        let mut id = id.borrow_mut();
        let current_id = id.get().clone();

        id.set(current_id + 1).unwrap()
    })
}

pub fn increment_total_supply() {
    TOTAL_SUPPLY.with(|s| {
        let mut s = s.borrow_mut();
        let current_supply = s.get().clone();
        s.set(current_supply + 1).unwrap();
    })
}

pub fn get_total_supply() -> u128 {
    TOTAL_SUPPLY.with(|s| s.borrow().get().clone())
}

pub fn tx_deduplication_check(
    permitted_past_time: u64,
    created_at_time: u64,
    memo: &Option<Memo>,
    id: u128,
    caller: &ICRCAccount,
    to: &ICRCAccount,
) -> Option<usize> {
    TRANSFER_LOG.with(|log_ref| {
        log_ref.borrow().iter().position(|log| {
            log.at > permitted_past_time
                && log.id == id
                && log.at == created_at_time
                && log.memo == *memo
                && log.from == *caller
                && log.to == *to
        })
    })
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Approval {
    pub expires_at: Option<u64>,
    pub account: ICRCAccount,
}

impl Storable for Approval {
    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: 100,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Approval {
    pub fn new(account: ICRCAccount, expires_at: Option<u64>) -> Self {
        Self {
            expires_at,
            account,
        }
    }
}

impl Storable for MintStatus {
    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: false,
        max_size: 90,
    };

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

pub fn calc_msgid(caller: &Subaccount, nonce: Nonce) -> u128 {
    let mut data = Vec::new();
    data.extend_from_slice(caller.as_slice());
    data.extend_from_slice(&nonce.to_le_bytes());
    let hashed = b3_utils::ledger::raw_sha256(&data);
    // Return XOR of 128 bit chunks of the hashed principal
    let mut id = 0;
    for i in 0..2 {
        id ^= u128::from_le_bytes(hashed[i * 16..(i + 1) * 16].try_into().unwrap_or_default());
    }
    id
}
