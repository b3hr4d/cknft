use ic_cdk_macros::{init, pre_upgrade, post_upgrade};
use candid::candid_method;
use ic_stable_structures::{writer::Writer, Memory};

use crate::{types::InitArg, state::{COLLECTION, Collection}};

#[init]
#[candid_method(init)]
pub fn init(
    arg: InitArg
){
    let authority = match arg.minting_authority{
        None => ic_cdk::caller(),
        Some(auth) => auth
    };
    COLLECTION.with(|c|{
        let mut c = c.borrow_mut();
        let collection = Collection{
            name: arg.name,
            symbol: arg.symbol,
            royalties: arg.royalties,
            minting_authority: authority,
            royalty_recipient: arg.royalties_recipient,
            description: arg.description,
            image: arg.image,
            supply_cap: arg.supply_cap,
            tx_window: arg.tx_window as u64 * 60 * 60 * 60 * 1000_000_000,
            permitted_drift: arg.permitted_drift as u64 * 60 * 60 * 1000_000_000,
            ..Default::default()
        };
        *c = collection;
    });
    // TX_WINDOW.with(|window|{
    //     let time = arg.tx_window as u64 * 60 * 60 * 60 * 1000_000_000;
    //     *window.borrow_mut() = time;
    // });
    // PERMITTED_DRIFT.with(|drift|{
    //     let time = arg.permitted_drift as u64 * 60 * 60 * 1000_000_000;
    //     *drift.borrow_mut() = time;
    // });
}

// A pre-upgrade hook for serializing the data stored on the heap.
#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    COLLECTION.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = crate::memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap();
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade() {
    let memory = crate::memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    COLLECTION.with(|s| {
        *s.borrow_mut() = state
    });
}