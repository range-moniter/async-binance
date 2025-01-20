use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SymbolBookTickerStream {
    symbol: Symbol,
}

impl SymbolBookTickerStream {
    pub fn new(symbol: Symbol) -> SymbolBookTickerStream {
        SymbolBookTickerStream { symbol }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}


impl StreamNameFormat for SymbolBookTickerStream {
    fn stream_name(&self) -> String {
        format!("{}@bookTicker", self.symbol.name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct TotalSymbolBookTickerStream;

impl StreamNameFormat for TotalSymbolBookTickerStream {
    fn stream_name(&self) -> String {
        "!bookTicker".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolBookTickerPayload {

    #[serde(rename = "u")]
    pub order_book_id: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "b")]
    pub best_bid_price: BigDecimal,
    #[serde(rename = "B")]
    pub best_bid_qty: BigDecimal,
    #[serde(rename = "a")]
    pub best_ask_price: BigDecimal,
    #[serde(rename = "A")]
    pub best_ask_qty: BigDecimal,

}