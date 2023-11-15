use candid::export_service;
use crate::{types::*, errors::*};
use ic_cdk_macros::query;
use icrc_ledger_types::{
    icrc1::account::Account, icrc::generic_metadata_value::MetadataValue
};

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        write(dir.join("icrc7.did"), export_candid()).expect("Write failed.");
    }
}