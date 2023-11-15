use candid::Principal;
use icrc_ledger_types::icrc1::account::{Account, DEFAULT_SUBACCOUNT};

pub fn default_account(owner: Principal) -> Account{
    Account { owner, subaccount: Some(DEFAULT_SUBACCOUNT.clone()) }
}

pub fn account_transformer(account: Account) -> Account{
    match account.subaccount{
        None => default_account(account.owner),
        Some(_) => account
    }
}