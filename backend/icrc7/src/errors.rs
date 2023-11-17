use candid::CandidType;

#[derive(CandidType, Clone)]
pub enum TransferError {
    Unauthorized { tokens_ids: Vec<u128> },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    TemporaryUnavailable,
    GenericError { error_code: u128, msg: String },
}

#[derive(CandidType, Clone)]
pub enum ApprovalError {
    Unauthorized { tokens_ids: Vec<u128> },
    TooOld,
    TemporaryUnavailable,
    GenericError { error_code: u128, msg: String },
}
