use crate::market::types::event_type::EventType;
use bigdecimal::BigDecimal;
use client::stream::stream::StreamNameFormat;
use general::symbol::Symbol;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AggTradeStream {
    symbol: Symbol,
}

impl AggTradeStream {
    pub fn new(symbol: Symbol) -> Self {
        Self { symbol }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl StreamNameFormat for AggTradeStream {
    fn stream_name(&self) -> String {
        format!("{}@aggTrade", self.symbol.name)
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AggTradeStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    #[serde(rename = "p")]
    pub price: BigDecimal,
    #[serde(rename = "q")]
    pub quantity: BigDecimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u64,
    #[serde(rename = "m")]
    pub is_buyer: bool,
    #[serde(rename = "M", default)]
    pub addition: bool,
}
