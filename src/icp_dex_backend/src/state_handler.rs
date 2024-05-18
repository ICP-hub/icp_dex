
use crate::types::{PoolArgs, UserData};
use crate::Memory;
use candid::Principal;
use ic_stable_structures::StableBTreeMap;

pub struct State {
    pub userdata: StableBTreeMap<Principal, UserData, Memory>,
    pub pooldata: StableBTreeMap<Principal, PoolArgs, Memory>,
}

impl State {
    pub fn new() -> Self {
        Self {
            userdata: init_user_data(),
            pooldata: init_pool_data(),
        }
    }
    
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn init_user_data() -> StableBTreeMap<Principal, UserData, Memory> {
    StableBTreeMap::init(crate::memory::get_postdata_memory())

}
fn init_pool_data() -> StableBTreeMap<Principal, PoolArgs, Memory> {
    StableBTreeMap::init(crate::memory::get_pool_data_memory())
}