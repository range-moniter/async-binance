use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TradeStream {
    symbol: Symbol,
}

impl TradeStream {
    pub fn new(symbol: Symbol) -> TradeStream {
        TradeStream { symbol }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}


impl StreamNameFormat for TradeStream {
    fn stream_name(&self) -> String {
        format!("{}@trade", self.symbol.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeStreamPayload {

    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p")]
    pub price: BigDecimal,
    #[serde(rename = "q")]
    pub quantity: BigDecimal,
    #[serde(rename = "T")]
    pub trade_time: u128,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(rename = "M", default)]
    pub addition: bool,
}