use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolMiniTickerStream {
    symbol: Symbol,
}
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AllSymbolMiniTickerStream;

impl SymbolMiniTickerStream {
    pub fn new(symbol: Symbol) -> SymbolMiniTickerStream {
        Self { symbol }
    }
}

impl StreamNameFormat for SymbolMiniTickerStream {
    fn stream_name(&self) -> String {
        format!("{}@miniTicker", self.symbol.name)
    }
}

impl StreamNameFormat for AllSymbolMiniTickerStream {
    fn stream_name(&self) -> String {
        "!miniTicker@arr".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolMiniTickerPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub close_price: BigDecimal,
    #[serde(rename = "o")]
    pub open_price: BigDecimal,
    #[serde(rename = "h")]
    pub high_price: BigDecimal,
    #[serde(rename = "l")]
    pub low_price: BigDecimal,
    #[serde(rename = "v")]
    pub total_base_volume: BigDecimal,
    #[serde(rename = "q")]
    pub total_quote_volume: BigDecimal,

}