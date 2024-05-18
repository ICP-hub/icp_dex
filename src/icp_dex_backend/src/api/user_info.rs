use crate::canister_types::{
    CanisterIdRecord, CanisterInstallMode, CreateCanisterArgument, CreateCanisterArgumentExtended,
    InstallCodeArgument, InstallCodeArgumentExtended,
};
use crate::types::{PoolArgs, UserData};
use crate::State;
use candid::{encode_one, CandidType, Principal};
use ic_cdk::api::call::{call_with_payment128, CallResult};
use ic_cdk::api::{ canister_version, time};
use serde::Deserialize;

pub fn set_post_data(state: &mut State, user_principal: Principal) -> Result<(), String> {
    let user_principal_text = user_principal.to_string();
    let data = UserData {
        user_principal: user_principal_text,
        created_at: time().to_string(),
    };
    let response = state.userdata.insert(user_principal, data);
    if let Some(_res) = response {
        Err("Not Able to upload data".to_string())
    } else {
        Ok(())
    }
}

pub fn get_post_data(state: &State, user_principal: Principal) -> Result<UserData, String> {
    let post_data = state.userdata.get(&user_principal);
    if let Some(post) = post_data {
        Ok(post)
    } else {
        Err("No Post Exist!".to_string())
    }
}

pub fn remove_user_post(state: &mut State, user_principal: Principal) -> Result<UserData, String> {
    let remove_success = state.userdata.remove(&user_principal);
    if let Some(remove) = remove_success {
        Ok(remove)
    } else {
        Err("no User found!".to_string())
    }
}

#[derive(CandidType, PartialEq, Deserialize, Debug, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}

pub async fn create_pool(
    state: &mut State,
    user_principal: Principal,
    arg: PoolArgs,
) -> Result<String, String> {
    let _usr_exist = state.userdata.get(&user_principal).expect("No User found!");

    let token_a_cid = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();
    let token_b_cid = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();

    let account = Account {
        owner: user_principal,
        subaccount: None,
    };

    //  ic_cdk::call<(account,r)>(token_a_cid, "icrc1_balance_of", (account,)).await.unwrap();                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  

    let pool_args_bytes: Vec<u8> = match encode_one(arg.clone()) {
        Ok(bytes) => bytes,
        Err(e) => return Err(format!("Failed to serialize DaoInput: {}", e)),
    };

    let arg = CreateCanisterArgument { settings: None };

    let (canister_id,) = match create_canister(arg).await {
        Ok(id) => id,
        Err((_, err_string)) => return Err(err_string),
    };

    let _addcycles = deposit_cycles(canister_id, 100000000).await;

    let canister_id_principal = canister_id.canister_id;

    println!("Canister ID: {}", canister_id_principal.to_string());

    let arg1 = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id_principal,
        wasm_module: vec![],
        arg: pool_args_bytes,
    };

    let _installcode = install_code(arg1).await;

    return Ok("Successfully created the pool!".to_string());
}

async fn create_canister(
    arg: CreateCanisterArgument,
    // cycles: u128,
) -> CallResult<(CanisterIdRecord,)> {
    let extended_arg = CreateCanisterArgumentExtended {
        settings: arg.settings,
        sender_canister_version: Some(canister_version()),
    };
    let cycles: u128 = 100_000_000_000;
    call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (extended_arg,),
        cycles,
    )
    .await
}

async fn deposit_cycles(arg: CanisterIdRecord, cycles: u128) -> CallResult<()> {
    call_with_payment128(
        Principal::management_canister(),
        "deposit_cycles",
        (arg,),
        cycles,
    )
    .await
}
async fn install_code(arg: InstallCodeArgument) -> CallResult<()> {
    // let wasm_base64: &str = "3831fb07143cd43c3c51f770342d2b7d0a594311529f5503587bf1544ccd44be";
    // let wasm_module_sample: Vec<u8> = base64::decode(wasm_base64).expect("Decoding failed");

    let wasm_module_sample: Vec<u8> = include_bytes!(
        "/home/anas/Documents/work/valueswap/icp_dex/.dfx/local/canisters/dex_pool/dex_pool.wasm"
    )
    .to_vec();

    let cycles: u128 = 10_000_000_000;

    let extended_arg = InstallCodeArgumentExtended {
        mode: arg.mode,
        canister_id: arg.canister_id,
        wasm_module: wasm_module_sample,
        arg: arg.arg,
        sender_canister_version: Some(canister_version()),
    };

    call_with_payment128(
        Principal::management_canister(),
        "install_code",
        (extended_arg,),
        cycles,
    )
    .await
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn get_principal() -> Principal {
//         Principal::from_text("bxquz-fu76r-igixs-bw537-mgkyg-h4goq-ldrwe-q4zg2-zxtqy-zupgm-nqe")
//             .unwrap()
//     }
//     fn generate_user_data() -> PostData {
//         let user_data = PostData {
//             title: "No".to_string(),
//             data: "New".to_string(),
//             created_by: "32334".to_string(),
//         };
//         return user_data;
//     }
//     #[test]
//     fn test_post_creation() {
//         let mut state = State::default();
//         let data = generate_user_data();
//         let response = state.set_post_data(get_principal(), data.clone());
//         match response {
//             Ok(res) => assert_eq!(res, ()),
//             Err(e) => assert_eq!(e, "Already uploaed the post of the User".to_string()),
//         }
//     }
//     #[test]
//     fn test_post_exist() {
//         let state = State::default();
//         let data = generate_user_data();
//         let user_data = state.get_post_data(get_principal());
//         match user_data {
//             Ok(res) => assert_eq!(res, data),
//             Err(e) => assert_eq!(e, "No data found".to_string()),
//         };
//     }
//     #[test]
//     fn test_post_exist_after_remove() {
//         let mut state = State::default();
//         let data = generate_user_data();
//         let user_data = state.remove_user_post(get_principal());
//         match user_data {
//             Ok(res) => assert_eq!(res, data),
//             Err(e) => assert_eq!(e, "no User found!".to_string()),
//         };
//     }
// }
