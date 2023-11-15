use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Decode, Encode, Nat, Principal};
use ic_stable_structures::{
    memory_manager::MemoryManager, BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable,
};
use icrc_ledger_types::{icrc::generic_metadata_value::MetadataValue, icrc1::account::Account};
use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};

use crate::{
    errors::{ApprovalError, TransferError},
    memory::init_stable_data,
    memory::Memory,
    types::{ApprovalArgs, CollectionMetadata, TransferArgs},
    utils::{account_transformer, default_account},
};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Approval {
    pub expires_at: Option<u64>,
    pub account: Account,
}

impl Storable for Approval {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Approval {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 100;
}

impl Approval {
    pub fn new(account: Account, expires_at: Option<u64>) -> Self {
        Self {
            expires_at,
            account,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Token {
    pub id: u128,
    pub owner: Account,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub description: Option<String>,
    pub approvals: Vec<Approval>,
}

impl Token {
    pub fn token_metadata(&self) -> Vec<(String, MetadataValue)> {
        let mut metadata = Vec::new();
        metadata.push(("Id".to_string(), MetadataValue::Nat(Nat::from(self.id))));
        metadata.push(("Name".into(), MetadataValue::Text(self.name.clone())));
        if self.image.is_some() {
            let buf = ByteBuf::from(self.image.as_ref().unwrap().clone());
            metadata.push(("Image".into(), MetadataValue::Blob(buf)))
        }
        if self.description.is_some() {
            let value = self.description.as_ref().unwrap().clone();
            metadata.push(("Description".into(), MetadataValue::Text(value)))
        }
        metadata
    }

    pub fn owner(&self) -> Account {
        self.owner.clone()
    }

    pub fn approval_check(&self, current_time: u64, account: &Account) -> bool {
        // self.approvals.iter().any(|approval| {
        //     ic_cdk::println!("owner: {:?} == {:?} && subaccount {:?} == {:?}", approval.account.owner, account.owner, approval.account.subaccount, account.subaccount);
        //     approval.account.owner == account.owner && approval.account.subaccount == account.subaccount
        //         && (approval.expires_at.is_none() || approval.expires_at >= Some(current_time))
        // })
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

    pub fn approve(&mut self, caller: &Account, approval: Approval) -> Result<(), ApprovalError> {
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
        caller: &Account,
        to: Account,
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
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Token {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 100000;
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TransferLog {
    pub id: u128,
    pub at: u64,
    pub memo: Option<Vec<u8>>,
    pub from: Account,
    pub to: Account,
}

impl Storable for TransferLog {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for TransferLog {
    const IS_FIXED_SIZE: bool = false;
    const MAX_SIZE: u32 = 200;
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    // name of the collection
    pub name: String,
    // symbol of the collection
    pub symbol: String,
    pub royalties: Option<u16>,
    // minting authority
    pub minting_authority: Principal,
    pub royalty_recipient: Option<Account>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub total_supply: u128,
    // max supply cap
    pub supply_cap: Option<u128>,
    #[serde(skip, default = "init_stable_data")]
    pub tokens: StableBTreeMap<u128, Token, Memory>,
    pub tx_count: u128,
    // #[serde(skip, default = "init_transfer_stable_data")]
    // pub transfer_log: StableBTreeMap<u128, TransferLog, Memory>,
    pub transfer_log: Vec<TransferLog>, // planning to replace Vector with StableBTreeMap
    pub tx_window: u64,
    pub permitted_drift: u64,
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            royalties: None,
            minting_authority: Principal::anonymous(),
            royalty_recipient: None,
            description: None,
            image: None,
            total_supply: 0,
            supply_cap: None,
            tokens: init_stable_data(),
            // transfer_log: init_transfer_stable_data(),
            transfer_log: Vec::new(),
            tx_count: 0,
            tx_window: 0,
            permitted_drift: 0,
        }
    }
}

impl Collection {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn royalties(&self) -> Option<u16> {
        self.royalties.clone()
    }

    pub fn royalty_recipient(&self) -> Option<Account> {
        self.royalty_recipient.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn image(&self) -> Option<String> {
        self.image.clone()
    }

    pub fn total_supply(&self) -> u128 {
        self.total_supply.clone()
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
            icrc7_total_supply: self.total_supply.clone(),
            icrc7_supply_cap: self.supply_cap.clone(),
        }
    }

    fn log_transfer_transaction(&mut self, log: TransferLog) {
        self.transfer_log.push(log);
    }

    pub fn mint(&mut self, caller: &Principal, token: Token) -> u128 {
        if *caller != self.minting_authority {
            ic_cdk::trap("Unauthorized Caller")
        }
        if let Some(ref cap) = self.supply_cap {
            if self.total_supply >= *cap {
                ic_cdk::trap("Supply Cap Reached")
            }
        }
        if self.tokens.contains_key(&token.id) {
            ic_cdk::trap("Id Exist")
        }
        self.total_supply += 1;
        self.tokens.insert(token.id.clone(), token);
        let id = self.get_tx_id();
        id
    }

    pub fn owner_of(&self, id: &u128) -> Account {
        match self.tokens.get(id) {
            None => ic_cdk::trap("Invalid Token Id"),
            Some(token) => token.owner.clone(),
        }
    }

    fn get_tx_id(&mut self) -> u128 {
        self.tx_count += 1;
        self.tx_count
    }

    pub fn tokens_of(&self, account: &Account) -> Vec<u128> {
        let mut ids = vec![];
        for (id, token) in self.tokens.iter() {
            if token.owner == *account {
                ids.push(id.clone())
            }
        }
        ids
    }

    pub fn token_metadata(&self, id: &u128) -> Vec<(String, MetadataValue)> {
        match self.tokens.get(id) {
            Some(token) => token.token_metadata(),
            None => ic_cdk::trap("Invalid Id"),
        }
    }

    pub fn balance_of(&self, account: &Account) -> u128 {
        let mut balance = 0;
        for (_, token) in self.tokens.iter() {
            if token.owner == *account {
                balance += 1;
                continue;
            }
        }
        balance
    }

    pub fn tx_deduplication_check(
        &self,
        permitted_past_time: u64,
        created_at_time: u64,
        memo: &Option<Vec<u8>>,
        id: u128,
        caller: &Account,
        to: &Account,
    ) -> Option<u128> {
        if let Some(index) = self
            .transfer_log
            .iter()
            .rev() // reversing
            .position(|log| {
                log.at > permitted_past_time
                    && log.id == id
                    && log.at == created_at_time
                    && log.memo == *memo
                    && log.from == *caller
                    && log.to == *to
            })
        {
            let index = self.transfer_log.len() - 1 - index;
            Some(index as u128)
        } else {
            None
        }
        // for (index, log) in self.transfer_log.iter().rev().enumerate(){
        //     if log.at 
        //     if log.id == id{
                
        //     }
        //     continue;
        // }
        // None
    }

    fn id_validity_check(&self, ids: &Vec<u128>) {
        let mut invalid_ids = vec![];
        for id in ids.iter() {
            match self.tokens.get(id) {
                Some(_) => continue,
                None => invalid_ids.push(id.clone()),
            }
        }
        if invalid_ids.len() > 0 {
            let error_msg = format!("Invalid Token Ids: {:?}", invalid_ids);
            ic_cdk::trap(&error_msg)
        }
    }

    pub fn transfer(
        &mut self,
        caller: &Principal,
        arg: TransferArgs,
    ) -> Result<u128, TransferError> {
        if arg.token_ids.len() == 0 {
            ic_cdk::trap("No Token Provided")
        }
        // checking if the token for respective ids is available or not
        self.id_validity_check(&arg.token_ids);
        let caller = account_transformer(Account { owner: caller.clone(), subaccount: arg.spender_subaccount });
        let to = account_transformer(arg.to);
        let current_time = ic_cdk::api::time();
        let mut tx_deduplication: HashMap<u128, TransferError> = HashMap::new();
        if let Some(arg_time) = arg.created_at_time {
            let permitted_past_time = current_time - self.tx_window - self.permitted_drift;
            let permitted_future_time = current_time + self.permitted_drift;
            if arg_time < permitted_past_time {
                return Err(TransferError::TooOld);
            }
            if arg_time > permitted_future_time {
                return Err(TransferError::CreatedInFuture {
                    ledger_time: current_time,
                });
            }
            //     match (
            //         permitted_past_time > arg_time,
            //         arg_time < permitted_future_time,
            //     ) {
            //         // checking if time is before permitted_past_time
            //         (false, _) => return Err(TransferError::TooOld),
            //         // checking if time is ahead of permitted_future_time
            //         (_, false) => {
            //             return Err(TransferError::CreatedInFuture {
            //                 ledger_time: current_time,
            //             })
            //         }
            //         (true, true) => ,
            //     }
            // }
            arg.token_ids.iter().for_each(|id| {
                if let Some(index) =
                    self.tx_deduplication_check(permitted_past_time, arg_time, &arg.memo, *id, &caller, &to)
                {
                    tx_deduplication.insert(
                        *id,
                        TransferError::Duplicate {
                            duplicate_of: index,
                        },
                    );
                }
            });
        }
        let mut unauthorized: Vec<u128> = vec![];
        arg.token_ids.iter().for_each(|id| {
            let token = match self.tokens.get(id) {
                None => ic_cdk::trap("Invalid Id"),
                Some(token) => token,
            };
            let approval_check =
                token.approval_check(current_time + self.permitted_drift, &caller);
            if token.owner != caller && !approval_check {
                unauthorized.push(id.clone())
            }
        });
        match arg.is_atomic {
            // when atomic transfer is turned off
            Some(false) => {
                for id in arg.token_ids.iter() {
                    if let Some(e) = tx_deduplication.get(id) {
                        return Err(e.clone());
                    }
                    let mut token = self.tokens.get(id).unwrap();
                    match token.transfer(current_time + self.permitted_drift, &caller, to) {
                        Err(_) => continue,
                        Ok(_) => {
                            let log = TransferLog {
                                id: id.clone(),
                                at: current_time,
                                memo: arg.memo.clone(),
                                from: caller,
                                to,
                            };
                            self.tokens.insert(id.clone(), token);
                            self.log_transfer_transaction(log);
                        }
                    }
                }
                if unauthorized.len() > 0 {
                    return Err(TransferError::Unauthorized {
                        tokens_ids: unauthorized,
                    });
                }
                Ok(self.get_tx_id())
            }
            // default behaviour of atomic
            _ => {
                for (_, e) in tx_deduplication.iter() {
                    return Err(e.clone());
                }
                if unauthorized.len() > 0 {
                    return Err(TransferError::Unauthorized {
                        tokens_ids: unauthorized,
                    });
                }
                for id in arg.token_ids.iter() {
                    let mut token = self.tokens.get(id).unwrap();
                    token.transfer(current_time + self.permitted_drift, &caller, to)?;
                    let log = TransferLog {
                        id: id.clone(),
                        at: current_time,
                        memo: arg.memo.clone(),
                        from: caller,
                        to,
                    };
                    self.tokens.insert(id.clone(), token);
                    self.log_transfer_transaction(log)
                }
                Ok(self.get_tx_id())
            }
        }
    }

    pub fn approve(
        &mut self,
        caller: &Principal,
        arg: ApprovalArgs,
    ) -> Result<u128, ApprovalError> {
        let caller = default_account(*caller);
        let token_ids = match arg.token_ids {
            None => self.tokens_of(&caller),
            Some(ids) => {
                self.id_validity_check(&ids);
                ids
            }
        };
        if token_ids.len() == 0 {
            ic_cdk::trap("No Tokens Available")
        }
        let approve_for = account_transformer(arg.spender);
        let approval = Approval {
            account: approve_for,
            expires_at: arg.expires_at,
        };
        for id in token_ids.iter() {
            let mut token = self.tokens.get(id).unwrap();
            token.approve(&caller, approval.clone())?;
            self.tokens.insert(id.clone(), token);
        }
        Ok(self.get_tx_id())
    }
}

thread_local! {
pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static COLLECTION: RefCell<Collection> = RefCell::default();
    // pub static TX_WINDOW: RefCell<u64> = RefCell::default();
    // pub static PERMITTED_DRIFT: RefCell<u64> = RefCell::default();
}

// pub fn get_tx_window() -> u64 {
//     COLLECTION.with(|c| c.borrow().tx_window.clone())
// }

// pub fn get_permitted_drift() -> u64 {
//     COLLECTION.with(|c| c.borrow().permitted_drift.clone())
// }