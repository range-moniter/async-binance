use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::symbol::Symbol;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AveragePriceStream {
    symbol: Symbol,
}

impl AveragePriceStream {
    pub fn new(symbol: Symbol) -> AveragePriceStream {
        AveragePriceStream { symbol }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
}

impl StreamNameFormat for AveragePriceStream {
    fn stream_name(&self) -> String {
        format!("{}@avgPrice", self.symbol.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AveragePricePayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub average_price_interval: String,
    #[serde(rename = "w")]
    pub average_price: BigDecimal,
    #[serde(rename = "T")]
    pub last_trade_time: u128
}