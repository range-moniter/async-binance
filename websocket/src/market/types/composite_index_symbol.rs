use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::symbol::Symbol;
use crate::market::types::event_type::EventType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct CompositionIndexSymbolStream {
    symbol: Symbol,
}

impl CompositionIndexSymbolStream {
    pub fn new(symbol: Symbol) -> Self {
        CompositionIndexSymbolStream {
            symbol,
        }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl StreamNameFormat for CompositionIndexSymbolStream {
    fn stream_name(&self) -> String {
        format!("{}@compositeIndex", self.symbol.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionIndexSymbolStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price: BigDecimal,
    #[serde(rename = "C")]
    pub asset_type: String,
    #[serde(rename = "c")]
    pub compositions: Vec<Composition>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Composition {
    #[serde(rename = "b")]
    pub base_asset: String,
    #[serde(rename = "q")]
    pub quote_asset: String,
    #[serde(rename = "w")]
    pub quantity_weight: BigDecimal,
    #[serde(rename = "W")]
    pub percentage_weight: BigDecimal,
    #[serde(rename = "i")]
    pub index_price: BigDecimal,
}
