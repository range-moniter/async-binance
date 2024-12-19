use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum EventType {
    #[serde(rename = "avgPrice")]
    AvgPrice,
    #[serde(rename = "depthUpdate")]
    DepthUpdate,
    #[serde(rename = "24hrTicker")]
    SymbolTicker24hr,
    #[serde(rename = "1hTicker")]
    TickerOneHour,
    #[serde(rename = "4hTicker")]
    TickerFourHours,
    #[serde(rename = "1dTicker")]
    TickerOneDay,
    #[serde(rename = "24hrMiniTicker")]
    MiniTicker24hr,
    #[serde(rename = "aggTrade")]
    AggTrade,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "kline")]
    Kline,
}
