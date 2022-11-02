use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Timestamp};
use cw_storage_plus::{Item, Map};

use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
}

#[cw_serde]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

#[cw_serde]
pub struct LockAddressInfo {
    pub lock_time: u64,
    pub balance: Uint128,
    pub expire: Option<u64>,
}
impl LockAddressInfo {
    pub fn is_expired(&self, block_time: Timestamp) -> bool {
        if let Some(expire) = self.expire {
            if expire <= block_time.seconds() {
                return true;
            }
        }
        false
    }
}

#[cw_serde]
pub struct LockAddressMsg {
    pub address: String,
    pub expire: Option<u64>,
}

#[cw_serde]
pub struct LockAddressResponse {
    pub operator: Addr,
    pub lock_address: Vec<Addr>,
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
// TODO: After https://github.com/CosmWasm/cw-plus/issues/670 is implemented, replace this with a `MultiIndex` over `ALLOWANCES`
pub const ALLOWANCES_SPENDER: Map<(&Addr, &Addr), AllowanceResponse> =
    Map::new("allowance_spender");

pub const LOCK_OPERATOR: Item<Addr> = Item::new("lock_operator");
pub const LOCK_ADDR_INFO: Map<&Addr, LockAddressInfo> = Map::new("lock_addr_info");
