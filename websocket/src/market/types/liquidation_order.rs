use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::enums::general::TimeInForce;
use general::enums::order::{OrderSide, OrderStatus, OrderType};
use general::symbol::Symbol;
use crate::market::types::event_type::EventType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct LiquidationOrderStream {
    symbol: Symbol,
}

impl LiquidationOrderStream {
    pub fn new(symbol: Symbol) -> LiquidationOrderStream {
        LiquidationOrderStream { symbol }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }

}

impl StreamNameFormat for LiquidationOrderStream {
    fn stream_name(&self) -> String {
        format!("{}@forceOrder", self.symbol.name)
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct TotalLiquidationOrderStream;

impl StreamNameFormat for TotalLiquidationOrderStream {
    fn stream_name(&self) -> String {
        "!forceOrder@arr".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationOrderStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "o")]
    pub payload: LiquidationOrderItem
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiquidationOrderItem {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "S")]
    pub order_side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    pub original_quantity: BigDecimal,
    #[serde(rename = "p")]
    pub price: BigDecimal,
    #[serde(rename = "ap")]
    pub average_price: BigDecimal,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "l")]
    pub last_filled_quantity: BigDecimal,
    #[serde(rename = "z")]
    pub filled_accumulated_quantity: BigDecimal,
    #[serde(rename = "T")]
    pub trade_time: u64
}
