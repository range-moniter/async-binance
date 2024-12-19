use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::enums::interval::Interval;
use general::enums::timezone::Timezone;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct KlineStream {
    symbol: Symbol,
    kline_type: Interval,
    timezone: Option<Timezone>,
}

impl KlineStream {
    pub fn new(symbol: Symbol, kline_type: Interval) -> Self {
        Self {
            symbol, kline_type, timezone: None,
        }
    }

    pub fn timezone(&mut self, timezone: Timezone){
        self.timezone = Some(timezone);
    }
}

impl StreamNameFormat for KlineStream {
    fn stream_name(&self) -> String {
        match &self.timezone {
            None => format!("{}@kline_{}", self.symbol.name, self.kline_type.as_str()),
            Some(timezone) => format!("{}@kline_{}", self.symbol.name, timezone.as_str()),
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: KlineBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KlineBody {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: Interval,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "o")]
    pub open_price: BigDecimal,
    #[serde(rename = "c")]
    pub close_price: BigDecimal,
    #[serde(rename = "h")]
    pub high_price: BigDecimal,
    #[serde(rename = "l")]
    pub low_price: BigDecimal,
    #[serde(rename = "v")]
    pub asset_volume: BigDecimal,
    #[serde(rename = "n")]
    pub trade_time: u64,
    #[serde(rename = "x")]
    pub is_kline_close: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: BigDecimal,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: BigDecimal,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: BigDecimal,
    #[serde(rename = "B")]
    pub addition: String,
}
