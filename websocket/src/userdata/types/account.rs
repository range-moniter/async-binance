use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountPositionEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "u")]
    pub last_change_time: u64,
    #[serde(rename = "B")]
    pub balances: Vec<Asset>,
}
#[derive(Debug, Deserialize)]
pub struct Asset {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    pub free: BigDecimal,
    #[serde(rename = "l")]
    pub locked: BigDecimal,
}