use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use crate::state::{MEMORY_MANAGER, Token, TransferLog};

// A memory for upgrades, where data from the heap can be serialized/deserialized.
const UPGRADES: MemoryId = MemoryId::new(0);

// A memory for the StableBTreeMap we're using. A new memory should be created for
// every additional stable structure.
const STABLE_BTREE: MemoryId = MemoryId::new(1);
const TRANSFER_BTREE: MemoryId = MemoryId::new(2);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub fn get_upgrades_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(UPGRADES))
}

pub fn get_token_stable_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STABLE_BTREE))
}

pub fn get_transfer_stable_btree_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TRANSFER_BTREE))
}

pub fn init_stable_data() -> StableBTreeMap<u128, Token, Memory> {
    StableBTreeMap::init(get_token_stable_btree_memory())
}

pub fn init_transfer_stable_data() -> StableBTreeMap<u128, TransferLog, Memory>{
    StableBTreeMap::init(get_transfer_stable_btree_memory())
}