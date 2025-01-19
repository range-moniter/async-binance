use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::symbol::Symbol;
use general::serialize_extend::from_str_to_f64;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolTickerStream {
    symbol: Symbol,
}

impl SymbolTickerStream {
    pub fn new(symbol: Symbol) -> SymbolTickerStream {
        SymbolTickerStream { symbol }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl StreamNameFormat for SymbolTickerStream {
    fn stream_name(&self) -> String {
        format!("{}@ticker", self.symbol.name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TotalSymbolTickerStream;

impl StreamNameFormat for TotalSymbolTickerStream {
    fn stream_name(&self) -> String {
        "!ticker@arr".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalSymbolTickerPayload(Vec<SymbolTickerPayload>);

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolTickerPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: BigDecimal,
    #[serde(rename = "P")]
    pub price_change_percent: BigDecimal,
    #[serde(rename = "w")]
    pub average_price: BigDecimal,
    #[serde(rename = "x")]
    pub first_trade_price: BigDecimal,
    #[serde(rename = "c")]
    pub last_price: BigDecimal,
    #[serde(rename = "Q")]
    pub last_quantity: BigDecimal,
    #[serde(rename = "b")]
    pub best_bid_price: BigDecimal,
    #[serde(rename = "B")]
    pub best_bid_quantity: BigDecimal,
    #[serde(rename = "a")]
    pub best_ask_price: BigDecimal,
    #[serde(rename = "A")]
    pub best_ask_quantity: BigDecimal,
    #[serde(rename = "o")]
    pub open_price: BigDecimal,
    #[serde(rename = "h")]
    pub high_price: BigDecimal,
    #[serde(rename = "l")]
    pub low_price: BigDecimal,
    #[serde(rename = "v", deserialize_with = "from_str_to_f64")]
    pub total_asset_volume: f64,
    #[serde(rename = "q", deserialize_with = "from_str_to_f64")]
    pub total_quote_volume: f64,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub total_trade_count: u64,
}
