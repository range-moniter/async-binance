use serde::{Deserialize, Serialize};
use general::enums::rate_limiter::RateLimiter;
use crate::types::market::asset::Asset;
use crate::types::market::symbol_info::FutureSymbol;

#[derive(Serialize, Deserialize, Debug)]
pub struct FutureExchangeInfoResp {
    #[serde(rename = "exchangeFilters")]
    pub exchange_filters: Vec<String>,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimiter>,
    #[serde(rename = "serverTime")]
    pub server_time: i64,
    pub assets: Vec<Asset>,
    pub symbols: Vec<FutureSymbol>,
    pub timezone: String,
}