use crate::{
    errors::{ApprovalError, TransferError},
    types::CollectionMetadata,
};
use b3_utils::{
    ledger::{ICRC1MetadataValue, ICRCAccount},
    memory::{
        init_stable_mem_refcell,
        types::{Bound, DefaultStableBTreeMap, DefaultStableCell, DefaultStableVec, Storable},
    },
};
use candid::{CandidType, Decode, Encode, Nat, Principal};
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    pub static CONFIG: RefCell<DefaultStableCell<CollectionConfig>> = init_stable_mem_refcell("config", 1).unwrap();
    pub static TOKENS: RefCell<DefaultStableBTreeMap<u128, Token>> = init_stable_mem_refcell("tokens", 2).unwrap();
    pub static TRANSFER_LOG: RefCell<DefaultStableVec<TransferLog>> = init_stable_mem_refcell("transfer_log", 3).unwrap();
    pub static TRANSACTION_ID: RefCell<DefaultStableCell<u128>> = init_stable_mem_refcell("transaction_id", 4).unwrap();
    pub static TOTAL_SUPPLY: RefCell<DefaultStableCell<u128>> = init_stable_mem_refcell("total_supply", 5).unwrap();
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CollectionConfig {
    pub name: String,
    pub symbol: String,
    pub royalties: Option<u16>,
    pub minting_authority: Principal,
    pub royalty_recipient: Option<ICRCAccount>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub supply_cap: Option<u128>,
    pub tx_window: u64,
    pub permitted_drift: u64,
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

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            royalties: None,
            minting_authority: Principal::anonymous(),
            royalty_recipient: None,
            description: None,
            image: None,
            supply_cap: None,
            tx_window: 0,
            permitted_drift: 0,
        }
    }
}

impl CollectionConfig {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn royalties(&self) -> Option<u16> {
        self.royalties.clone()
    }

    pub fn royalty_recipient(&self) -> Option<ICRCAccount> {
        self.royalty_recipient.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn image(&self) -> Option<String> {
        self.image.clone()
    }

    pub fn supply_cap(&self) -> Option<u128> {
        self.supply_cap.clone()
    }

    pub fn metadata(&self) -> CollectionMetadata {
        CollectionMetadata {
            icrc7_name: self.name.clone(),
            icrc7_symbol: self.symbol.clone(),
            icrc7_royalties: self.royalties.clone(),
            icrc7_royalty_recipient: self.royalty_recipient.clone(),
            icrc7_description: self.description.clone(),
            icrc7_image: self.image.clone(),
            icrc7_total_supply: get_total_supply(),
            icrc7_supply_cap: self.supply_cap.clone(),
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
            return Err(TransferError::Unauthorized {
                tokens_ids: vec![self.id],
            });
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

pub fn id_validity_check(ids: &Vec<u128>) {
    let mut invalid_ids = vec![];

    TOKENS.with(|tokens| {
        for id in ids.iter() {
            match tokens.borrow().get(id) {
                Some(_) => continue,
                None => invalid_ids.push(id.clone()),
            }
        }
    });

    if invalid_ids.len() > 0 {
        let error_msg = format!("Invalid Token Ids: {:?}", invalid_ids);
        ic_cdk::trap(&error_msg)
    }
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TransferLog {
    pub id: u128,
    pub at: u64,
    pub memo: Option<Vec<u8>>,
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
    memo: &Option<Vec<u8>>,
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
