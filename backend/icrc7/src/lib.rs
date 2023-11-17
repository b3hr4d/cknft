pub mod errors;
pub mod state;
pub mod types;

use crate::types::{CollectionMetadata, Standard};
use crate::{
    errors::{ApprovalError, TransferError},
    state::Token,
    state::{CollectionConfig, CONFIG},
    types::{ApprovalArgs, MintArgs, TransferArgs},
};
use b3_utils::http::{HttpRequest, HttpResponse, HttpResponseBuilder};
use b3_utils::ledger::{ICRC1MetadataValue, ICRCAccount};
use b3_utils::memory::with_stable_mem;
use ic_cdk::{init, query, update};
use state::{
    get_total_supply, id_validity_check, increment_total_supply, increment_tx_id,
    tx_deduplication_check, Approval, TransferLog, TOKENS, TOTAL_SUPPLY, TRANSFER_LOG,
};
use std::collections::HashMap;

#[init]
pub fn init(arg: CollectionConfig) {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();

        c.set(arg).unwrap();
    });
}

/// ======== Query ========

#[query]
pub fn icrc7_name() -> String {
    CONFIG.with(|c| c.borrow().get().name())
}

#[query]
pub fn icrc7_symbol() -> String {
    CONFIG.with(|c| c.borrow().get().symbol())
}

#[query]
pub fn icrc7_royalties() -> Option<u16> {
    CONFIG.with(|c| c.borrow().get().royalties())
}

#[query]
pub fn icrc7_royalty_recipient() -> Option<ICRCAccount> {
    CONFIG.with(|c| c.borrow().get().royalty_recipient())
}

#[query]
pub fn icrc7_description() -> Option<String> {
    CONFIG.with(|c| c.borrow().get().description())
}

#[query]
pub fn icrc7_image() -> Option<String> {
    CONFIG.with(|c| c.borrow().get().image())
}

#[query]
pub fn icrc7_total_supply() -> u128 {
    TOTAL_SUPPLY.with(|s| s.borrow().get().clone())
}

#[query]
pub fn icrc7_supply_cap() -> Option<u128> {
    CONFIG.with(|c| c.borrow().get().supply_cap())
}

#[query]
pub fn icrc7_collection_metadata() -> CollectionMetadata {
    CONFIG.with(|c| c.borrow().get().metadata())
}

#[query]
pub fn icrc7_metadata(id: u128) -> Vec<(String, ICRC1MetadataValue)> {
    match TOKENS.with(|tokens| tokens.borrow().get(&id)) {
        None => ic_cdk::trap("Invalid Token Id"),
        Some(token) => token.token_metadata(),
    }
}

#[query]
pub fn icrc7_owner_of(id: u128) -> ICRCAccount {
    TOKENS.with(|tokens| {
        let tokens = tokens.borrow();
        match tokens.get(&id) {
            None => ic_cdk::trap("Invalid Token Id"),
            Some(token) => token.owner.clone(),
        }
    })
}

#[query]
pub fn icrc7_balance_of(account: ICRCAccount) -> u128 {
    let mut balance = 0;

    TOKENS.with(|tokens| {
        for (_, token) in tokens.borrow().iter() {
            if token.owner == account {
                balance += 1;
                continue;
            }
        }
    });

    balance
}

#[query]
pub fn icrc7_tokens_of(account: ICRCAccount) -> Vec<u128> {
    let mut ids = vec![];
    TOKENS.with(|tokens| {
        for (id, token) in tokens.borrow().iter() {
            if token.owner == account {
                ids.push(id.clone())
            }
        }
    });

    ids
}

#[query]
pub fn icrc7_supported_standards() -> Vec<Standard> {
    vec![Standard {
        name: "ICRC-7".into(),
        url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".into(),
    }]
}

/// ======== Update ========

#[update]
pub fn icrc7_transfer(arg: TransferArgs) -> Result<u128, TransferError> {
    if arg.token_ids.len() == 0 {
        ic_cdk::trap("No Token Provided")
    }
    // checking if the token for respective ids is available or not
    id_validity_check(&arg.token_ids);

    let caller = ICRCAccount::new(ic_cdk::caller(), arg.spender_subaccount);

    let current_time = ic_cdk::api::time();
    let mut tx_deduplication: HashMap<u128, TransferError> = HashMap::new();

    CONFIG.with(|c| {
        let c = c.borrow();
        let config = c.get();

        if let Some(arg_time) = arg.created_at_time {
            let permitted_past_time = current_time - config.tx_window - config.permitted_drift;
            let permitted_future_time = current_time + config.permitted_drift;

            if arg_time < permitted_past_time {
                return Err(TransferError::TooOld);
            }
            if arg_time > permitted_future_time {
                return Err(TransferError::CreatedInFuture {
                    ledger_time: current_time,
                });
            }

            arg.token_ids.iter().for_each(|id| {
                if let Some(index) = tx_deduplication_check(
                    permitted_past_time,
                    arg_time,
                    &arg.memo,
                    *id,
                    &caller,
                    &arg.to,
                ) {
                    tx_deduplication.insert(
                        *id,
                        TransferError::Duplicate {
                            duplicate_of: index as u128,
                        },
                    );
                }
            });
        }

        let mut unauthorized: Vec<u128> = vec![];
        arg.token_ids.iter().for_each(|id| {
            let token = match TOKENS.with(|tokens| tokens.borrow().get(id)) {
                None => ic_cdk::trap("Invalid Id"),
                Some(token) => token,
            };

            let approval_check =
                token.approval_check(current_time + config.permitted_drift, &caller);
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
                    let mut token = TOKENS.with(|tokens| tokens.borrow().get(id).unwrap());

                    match token.transfer(
                        current_time + config.permitted_drift,
                        &caller,
                        arg.to.clone(),
                    ) {
                        Err(_) => continue,
                        Ok(_) => {
                            let log = TransferLog {
                                id: id.clone(),
                                at: current_time,
                                memo: arg.memo.clone(),
                                from: caller.clone(),
                                to: arg.to.clone(),
                            };
                            TOKENS.with(|tokens| tokens.borrow_mut().insert(id.clone(), token));

                            TRANSFER_LOG.with(|log_ref| log_ref.borrow_mut().push(&log).unwrap());
                        }
                    }
                }
                if unauthorized.len() > 0 {
                    return Err(TransferError::Unauthorized {
                        tokens_ids: unauthorized,
                    });
                }

                Ok(increment_tx_id())
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
                    let mut token = TOKENS.with(|tokens| tokens.borrow().get(id).unwrap());
                    token.transfer(
                        current_time + config.permitted_drift,
                        &caller,
                        arg.to.clone(),
                    )?;
                    let log = TransferLog {
                        id: id.clone(),
                        at: current_time,
                        memo: arg.memo.clone(),
                        from: caller.clone(),
                        to: arg.to.clone(),
                    };

                    TOKENS.with(|tokens| tokens.borrow_mut().insert(id.clone(), token));
                    TRANSFER_LOG.with(|log_ref| log_ref.borrow_mut().push(&log).unwrap());
                }

                Ok(increment_tx_id())
            }
        }
    })
}

#[update]
pub fn icrc7_approve(arg: ApprovalArgs) -> Result<u128, ApprovalError> {
    let caller = ICRCAccount::from(ic_cdk::caller());

    let token_ids = match arg.token_ids {
        None => icrc7_tokens_of(caller.clone()),
        Some(ids) => {
            id_validity_check(&ids);
            ids
        }
    };

    if token_ids.len() == 0 {
        ic_cdk::trap("No Tokens Available")
    }
    let approve_for = ICRCAccount::from(arg.spender);
    let approval = Approval {
        account: approve_for,
        expires_at: arg.expires_at,
    };

    TOKENS.with(|tokens| {
        for id in token_ids.iter() {
            let mut token = tokens.borrow().get(id).unwrap();
            token.approve(&caller, approval.clone())?;
            tokens.borrow_mut().insert(id.clone(), token);
        }

        Ok(increment_tx_id())
    })
}

#[update]
pub fn icrc7_mint(arg: MintArgs) -> u128 {
    let token = Token {
        id: arg.id,
        name: arg.name,
        description: arg.description,
        image: arg.image,
        owner: arg.to,
        approvals: Vec::new(),
    };

    CONFIG.with(|c| {
        let c = c.borrow();
        let config = c.get();

        if ic_cdk::caller() != config.minting_authority {
            ic_cdk::trap("Unauthorized Caller")
        }

        if let Some(cap) = config.supply_cap {
            if cap < get_total_supply() {
                ic_cdk::trap("Supply Cap Reached")
            }
        }

        if TOKENS.with(|tokens| tokens.borrow().contains_key(&token.id)) {
            ic_cdk::trap("Id Exist")
        }

        increment_total_supply();

        TOKENS.with(|tokens| tokens.borrow_mut().insert(token.id, token));

        increment_tx_id()
    })
}

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    match req.path() {
        "/token" => {
            let token_id = req.raw_query_param("id").unwrap();

            let token = TOKENS.with(|tokens| {
                let tokens = tokens.borrow();
                match tokens.get(&token_id.parse::<u128>().unwrap_or_default()) {
                    None => ic_cdk::trap("Invalid Token Id"),
                    Some(token) => token,
                }
            });

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&token).unwrap_or_default())
                .build()
        }
        "/partition_details" => {
            let list = with_stable_mem(|pm| pm.partition_details());

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&list).unwrap_or_default())
                .build()
        }
        "/transfer_log" => {
            let transfer_id = req.raw_query_param("id").unwrap();

            let tx_logs = TRANSFER_LOG.with(|logs| {
                let logs = logs.borrow();
                match logs.get(transfer_id.parse::<u64>().unwrap_or_default()) {
                    None => ic_cdk::trap("Invalid Transfer Id"),
                    Some(log) => log,
                }
            });

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&tx_logs).unwrap_or_default())
                .build()
        }
        _ => HttpResponseBuilder::not_found().build(),
    }
}

ic_cdk::export_candid!();
