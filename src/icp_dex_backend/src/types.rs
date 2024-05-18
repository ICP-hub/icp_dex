use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use std::borrow::Cow;

pub struct Account {
    pub owner: String,
    pub subaccount: Option<serde_bytes::ByteBuf>,
}
#[derive(CandidType, PartialEq, Deserialize, Debug, Clone)]
pub struct UserData {
    pub user_principal: String,
    pub created_at: String,
}

#[derive(CandidType, PartialEq, Deserialize, Debug, Clone)]

pub struct TokenMetaData {
    pub name: String,
    pub canister_id: String,
    pub amount: u128,
}
#[derive(CandidType, PartialEq, Deserialize, Debug, Clone)]

pub struct TokenData {
    pub token_a: TokenMetaData,
    pub token_b: TokenMetaData,
}
#[derive(CandidType, PartialEq, Deserialize, Debug, Clone)]

pub struct PoolArgs {
    pub name: String,
    pub symbol: String,
    pub tokens: TokenData,
    pub ratio: (u8, u8),
    pub asset_managers: Vec<String>,
    pub swap_fee_percentage: f32,
}

const MAX_VALUE_SIZE: u32 = 100;

impl Storable for UserData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

impl Storable for PoolArgs {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
