use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, BlockInfo};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub fee_address: Addr,
    pub native_token: String,
    pub enabled: bool,
    pub collection_address: Addr,
    pub duration: u64,
}

#[cw_serde]
pub struct  NftInfo {
    pub nft_id: String,
    pub lock_time: u64,
    pub airdrop: Uint128,
    pub collection_address: Addr,
}

#[cw_serde]
pub struct UserInfo {
    pub address: Addr,
    pub total_earnd: Uint128,
    pub staked_nfts: Vec<NftInfo>,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const CURRENT_AIRDROP_KEY: &str = "current_airdrop";
pub const CURRENT_AIRDROP: Item<BlockInfo> = Item::new(CURRENT_AIRDROP_KEY);

pub const START_AIRDROP_KEY: &str = "start_airdrop";
pub const START_AIRDROP: Item<bool> = Item::new(START_AIRDROP_KEY);

pub const TOTAL_AIRDROP_KEY: &str = "total_airdrop";
pub const TOTAL_AIRDROP: Item<Uint128> = Item::new(TOTAL_AIRDROP_KEY);

pub const TOTAL_STAKED_KEY: &str = "total_staked";
pub const TOTAL_STAKED: Item<u64> = Item::new(TOTAL_STAKED_KEY);

pub const LOCKTIME_FEE_KEY: &str = "locktime_fee";
pub const LOCKTIME_FEE: Item<Uint128> = Item::new(LOCKTIME_FEE_KEY);

pub const ACCOUNT_MAP_PREFIX: &str = "account_map";
pub const ACCOUNT_MAP: Map<Addr, UserInfo> = Map::new(ACCOUNT_MAP_PREFIX);
