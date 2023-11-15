use candid::{CandidType, Principal, Deserialize, Encode, Nat, candid_method, export_service};
use ic_cdk_macros::*;
use icrc_ledger_types::icrc1::account::Account;
use ic_cdk::api::management_canister::main::{
    CreateCanisterArgument, CanisterIdRecord, CanisterSettings, CanisterInstallMode, InstallCodeArgument,
};

#[derive(CandidType)]
pub struct InitArg{
    pub name: String,
    pub symbol: String,
    pub minting_authority: Option<Principal>,
    pub royalties: Option<u16>,
    pub royalties_recipient: Option<Account>,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub supply_cap: Option<u128>,
}

#[derive(CandidType, Deserialize)]
pub struct CreateArg{
    pub name: String,
    pub symbol: String,
    pub royalties: Option<u16>,
    pub royalties_recipient: Option<Account>,
    pub description: Option<String>,
    pub image: Option<Vec<u8>>,
    pub supply_cap: Option<u128>,
}

impl From<(Principal, CreateArg)> for InitArg{
    fn from((minting_authority, arg): (Principal, CreateArg)) -> Self {
        InitArg { name: arg.name, symbol: arg.symbol, minting_authority: Some(minting_authority), royalties: arg.royalties, royalties_recipient: arg.royalties_recipient, description: arg.description, image: arg.image, supply_cap: arg.supply_cap }
    }
}

const WASM: &[u8] = std::include_bytes!("./../../../target/wasm32-unknown-unknown/release/icrc7.wasm.gz");

pub async fn get_an_address(caller: &Principal) -> Principal{
    ic_cdk::println!("{}", caller.clone());
    let canister_setting = CanisterSettings{
        controllers: Some(vec![caller.clone(), ic_cdk::id()]),
        compute_allocation: Some(Nat::from(0_u64)),
        memory_allocation: Some(Nat::from(0_u64)),
        freezing_threshold: Some(Nat::from(0_u64)),
    };
    let args = CreateCanisterArgument{
        settings: Some(canister_setting)
    };
    let (canister_id,): (CanisterIdRecord,) = match ic_cdk::api::call::call_with_payment(Principal::management_canister(), "create_canister", (args,), 200_000_000_000).await
    {
        Ok(x) => x,
        Err((_, _)) => (CanisterIdRecord{
            canister_id: candid::Principal::anonymous()
        },),
    };
    canister_id.canister_id
}

pub async fn install_wasm(wasm: Vec<u8>, canister_id: Principal, args: Vec<u8>,) -> bool{
    let install_config = InstallCodeArgument{
        mode: CanisterInstallMode::Install,
        wasm_module: wasm,
        canister_id,
        arg: args
    };
    match ic_cdk::api::call::call(Principal::management_canister(), "install_code", (install_config,)).await
    {
        Ok(x) => x,
        Err((rejection_code, msg)) =>{
            ic_cdk::println!("{:?} {:?}", rejection_code, msg);
            return false
        }
    }
    true
}

#[update]
#[candid_method(update)]
pub async fn create_icrc7_collection(
    arg: CreateArg
) -> Principal{
    let caller = ic_cdk::caller();
    let arg = InitArg::from((caller, arg));
    let address = get_an_address(&caller).await;
    if address == Principal::anonymous(){
        ic_cdk::trap("Failed to get an address")
    }
    let arg = Encode!(&arg).unwrap();
    match install_wasm(WASM.to_vec(), address, arg).await{
        true => address,
        false => ic_cdk::trap("Failed to install code")
    }
}

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
        write(dir.join("service.did"), export_candid()).expect("Write failed.");
    }
}