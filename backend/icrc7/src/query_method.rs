use ic_cdk_macros::query;
use candid::candid_method;
use icrc_ledger_types::{icrc1::account::Account, icrc::generic_metadata_value::MetadataValue};

use crate::{state::COLLECTION, types::{CollectionMetadata, Standard}, utils::account_transformer};

#[query]
#[candid_method(query)]
pub fn icrc7_name() -> String{
    COLLECTION.with(|c| c.borrow().name())
}

#[query]
#[candid_method(query)]
pub fn icrc7_symbol() -> String{
    COLLECTION.with(|c| c.borrow().symbol())
}

#[query]
#[candid_method(query)]
pub fn icrc7_royalties() -> Option<u16>{
    COLLECTION.with(|c| c.borrow().royalties())
}

#[query]
#[candid_method(query)]
pub fn icrc7_royalty_recipient() -> Option<Account>{
    COLLECTION.with(|c| c.borrow().royalty_recipient())
}

#[query]
#[candid_method(query)]
pub fn icrc7_description() -> Option<String>{
    COLLECTION.with(|c| c.borrow().description())
}

#[query]
#[candid_method(query)]
pub fn icrc7_image() -> Option<String>{
    COLLECTION.with(|c| c.borrow().image())
}

#[query]
#[candid_method(query)]
pub fn icrc7_total_supply() -> u128{
    COLLECTION.with(|c| c.borrow().total_supply())
}

#[query]
#[candid_method(query)]
pub fn icrc7_supply_cap() -> Option<u128>{
    COLLECTION.with(|c| c.borrow().supply_cap())
}

#[query]
#[candid_method(query)]
pub fn icrc7_collection_metadata() -> CollectionMetadata{
    COLLECTION.with(|c| c.borrow().metadata())
}

#[query]
#[candid_method(query)]
pub fn icrc7_metadata(id: u128) -> Vec<(String, MetadataValue)>{
    COLLECTION.with(|c| c.borrow().token_metadata(&id))
} 

#[query]
#[candid_method(query)]
pub fn icrc7_owner_of(id: u128) -> Account{
    COLLECTION.with(|collection| collection.borrow().owner_of(&id))
}

#[query]
#[candid_method(query)]
pub fn icrc7_balance_of(account: Account) -> u128{
    let account = account_transformer(account);
    COLLECTION.with(|collection| collection.borrow().balance_of(&account))
}

#[query]
#[candid_method(query)]
pub fn icrc7_tokens_of(account: Account) -> Vec<u128>{
    let account = account_transformer(account);
    COLLECTION.with(|collection| collection.borrow().tokens_of(&account))
}

#[query]
#[candid_method(query)]
pub fn icrc7_supported_standards() -> Vec<Standard>{
    vec![
        Standard{
            name: "ICRC-7".into(),
            url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".into()
        }
    ]
}