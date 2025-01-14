use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::enums::window_size::WindowSize;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolRollingWindowStream {
    symbol: Symbol,
    window_size: WindowSize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AllSymbolRollingStream{
    window_size: WindowSize,
}


impl SymbolRollingWindowStream {
    pub fn new(symbol: Symbol, window_size: WindowSize) -> Self {
        Self { symbol, window_size }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
    pub fn get_window_size(&self) -> WindowSize {
        self.window_size.clone()
    }
}


impl AllSymbolRollingStream {
    pub fn new(window_size: WindowSize) -> Self {
        Self { window_size }
    }
}
impl StreamNameFormat for SymbolRollingWindowStream {
    fn stream_name(&self) -> String {
        format!("{}@ticker_{}", self.symbol.name, self.window_size.as_str())
    }
}


impl StreamNameFormat for AllSymbolRollingStream {
    fn stream_name(&self) -> String {
        format!("!ticker_{}@arr", self.window_size.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolRollingPayload {
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
    #[serde(rename = "o")]
    pub open_price: BigDecimal,
    #[serde(rename = "h")]
    pub high_price: BigDecimal,
    #[serde(rename = "l")]
    pub low_price: BigDecimal,
    #[serde(rename = "c")]
    pub last_price: BigDecimal,
    #[serde(rename = "w")]
    pub weight_trade_asset_price: BigDecimal,
    #[serde(rename = "v")]
    pub total_traded_asset_volume: BigDecimal,
    #[serde(rename = "q")]
    pub total_traded_quote_volume: BigDecimal,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub total_trade_number: u64,
}