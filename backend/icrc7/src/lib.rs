pub mod crypto;
pub mod errors;
pub mod state;
pub mod types;

use crate::crypto::EcdsaSignature;
use crate::state::{calc_msgid, PUBLIC_KEY, SIGNATURE_MAP, STATUS_MAP};
use crate::types::{CollectionMetadata, MintState, MintStatus, Standard};
use crate::{
    errors::{ApprovalError, TransferError},
    state::Token,
    state::{CollectionConfig, CONFIG},
    types::{ApprovalArgs, MintArgs, TransferArgs},
};
use b3_utils::http::{HttpRequest, HttpResponse, HttpResponseBuilder};
use b3_utils::ledger::{raw_keccak256, ICRC1MetadataValue, ICRCAccount};
use b3_utils::memory::with_stable_mem;
use b3_utils::nonce::Nonce;
use b3_utils::{caller_is_controller, vec_to_hex_string_with_0x};
use b3_utils::{hex_string_with_0x_to_vec, Subaccount};
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
    EcdsaPublicKeyResponse, SignWithEcdsaArgument, SignWithEcdsaResponse,
};
use ic_cdk::{init, query, update};
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use state::{
    get_icrc7_config, get_total_supply, id_validity_check, increment_total_supply, increment_tx_id,
    tx_deduplication_check, Approval, TransferLog, NONCE_MAP, TOKENS, TOTAL_SUPPLY, TRANSFER_LOG,
};
use std::collections::HashMap;
use types::SelfMintArgs;

#[init]
pub fn init(arg: CollectionConfig) {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();

        c.set(arg).unwrap();
    });
}

/// ======== Query ========

#[query]
pub fn icrc7_public_key() -> Vec<u8> {
    PUBLIC_KEY.with(|pk| pk.borrow().get().clone())
}

#[query]
pub fn ethereum_address() -> String {
    let public_key = PUBLIC_KEY.with(|pk| pk.borrow().get().clone());

    let uncompressed_pubkey = VerifyingKey::from_sec1_bytes(&public_key)
        .unwrap()
        //.unwrap()
        .to_encoded_point(false);
    let ethereum_pubkey = &uncompressed_pubkey.as_bytes()[1..]; // trim off the first 0x04 byte

    let hashed_payload = raw_keccak256(&ethereum_pubkey).to_vec();

    vec_to_hex_string_with_0x(&hashed_payload[12..32])
}

#[query]
pub fn icrc7_config() -> CollectionConfig {
    CONFIG.with(|c| c.borrow().get().clone())
}

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

    let config = get_icrc7_config();

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

        let approval_check = token.approval_check(current_time + config.permitted_drift, &caller);
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

    let config = get_icrc7_config();

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

#[update]
pub fn update_config(arg: CollectionConfig) {
    CONFIG.with(|c| {
        let mut c = c.borrow_mut();

        c.set(arg).unwrap();
    });
}

#[update]
pub async fn mint_cknft(id: u64, chain_id: u64, target_eth_wallet: String) -> SelfMintArgs {
    let caller = ic_cdk::caller();
    let caller_subaccount = Subaccount::from(caller);

    let nonce = NONCE_MAP.with(|nonce_map| {
        let mut nonce_map = nonce_map.borrow_mut();
        let nonce = nonce_map
            .get(&caller_subaccount)
            .unwrap_or(Nonce::zero())
            .add_64(1);

        nonce_map.insert(caller_subaccount.clone(), nonce);
        nonce
    });

    let msg_id = calc_msgid(&caller_subaccount, nonce);
    let config = get_icrc7_config();
    let now = ic_cdk::api::time();
    let expiry = now / 1_000_000_000 + config.tx_window;

    fn update_status(msg_id: u128, id: u64, expiry: u64, state: MintState) {
        STATUS_MAP.with(|sm| {
            let mut sm = sm.borrow_mut();
            sm.insert(
                msg_id,
                MintStatus {
                    id,
                    amount: 1,
                    expiry,
                    state,
                },
            );
        });
    }

    update_status(msg_id, id, expiry, MintState::Init);

    // transfer CKNFT to this canister
    let transfer_args = TransferArgs {
        to: ICRCAccount::from(ic_cdk::id()),
        from: ICRCAccount::from(caller),
        token_ids: vec![id as u128],
        memo: None,
        is_atomic: None,
        created_at_time: None,
        spender_subaccount: None,
    };

    icrc7_transfer(transfer_args).unwrap();

    // Generate tECDSA signature
    // payload is (amount, to, msgId, expiry, chainId, cknft_eth_address), 32 bytes each
    let cknft_eth_address = hex_string_with_0x_to_vec(&config.cknft_eth_address).unwrap();

    let mut payload_to_sign: [u8; 192] = [0; 192];
    payload_to_sign[24..32].copy_from_slice(&u64::from(id.clone()).to_be_bytes());
    payload_to_sign[44..64]
        .copy_from_slice(&hex_string_with_0x_to_vec(&target_eth_wallet).unwrap());
    payload_to_sign[80..96].copy_from_slice(&msg_id.to_be_bytes());
    payload_to_sign[120..128].copy_from_slice(&expiry.to_be_bytes());
    payload_to_sign[152..160].copy_from_slice(&chain_id.to_be_bytes());
    payload_to_sign[172..192].copy_from_slice(&cknft_eth_address);

    let hashed_payload = raw_keccak256(&payload_to_sign).to_vec();

    let args = SignWithEcdsaArgument {
        derivation_path: vec![],
        message_hash: hashed_payload.clone(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: config.ecdsa_key_name,
        },
    };

    let signature: Vec<u8> = {
        let (res,): (SignWithEcdsaResponse,) = sign_with_ecdsa(args)
            .await
            .unwrap_or_else(|e| ic_cdk::trap(&format!("Failed to sign with ecdsa: {}", e.1)));
        res.signature
    };

    let sec1_public_key = PUBLIC_KEY.with(|pk| pk.borrow().get().clone());

    let public_key = VerifyingKey::from_sec1_bytes(&sec1_public_key).unwrap();

    let recid = RecoveryId::trial_recovery_from_prehash(
        &public_key,
        &hashed_payload,
        &Signature::from_slice(signature.as_slice()).unwrap(),
    )
    .unwrap();

    let v = recid.is_y_odd() as u8 + 27;

    SIGNATURE_MAP.with(|sm| {
        let mut sm = sm.borrow_mut();
        sm.insert(msg_id, EcdsaSignature::from_signature_v(&signature, v));
    });

    update_status(msg_id, id, expiry, MintState::Signed);

    // Return tECDSA signature
    SelfMintArgs {
        id,
        to: target_eth_wallet,
        msgid: msg_id,
        expiry,
        signature: EcdsaSignature::from_signature_v(&signature, v).to_string(),
    }
}

#[update(guard = "caller_is_controller")]
pub async fn update_ckicp_state() -> Vec<u8> {
    let config = get_icrc7_config();

    let args = EcdsaPublicKeyArgument {
        canister_id: None,
        derivation_path: vec![],
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: config.ecdsa_key_name,
        },
    };

    // Update tecdsa signer key and calculate signer ETH address
    let (res,): (EcdsaPublicKeyResponse,) = ecdsa_public_key(args)
        .await
        .unwrap_or_else(|_| ic_cdk::trap("Failed to get ecdsa public key"));

    PUBLIC_KEY.with(|pk| {
        let mut pk = pk.borrow_mut();
        pk.set(res.public_key.clone()).unwrap();
    });

    res.public_key
}

ic_cdk::export_candid!();
