use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BalanceEvent {
    #[serde(rename = "E")]
    pub event_time: i64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    pub balance_delta: String,
    #[serde(rename = "T")]
    pub clear_time: i64,
}