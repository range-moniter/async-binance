use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::symbol::Symbol;
use crate::market::types::event_type::EventType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MarkPriceStream {
    symbol: Symbol,
    is_second: bool,
}

impl MarkPriceStream {
    pub fn new(symbol: Symbol, is_second: bool) -> MarkPriceStream {
        MarkPriceStream { symbol, is_second }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }

    pub fn is_second(&self) -> bool {
        self.is_second
    }
}


impl StreamNameFormat for MarkPriceStream {
    fn stream_name(&self) -> String {
        if self.is_second {
            format!("{}@markPrice", self.symbol.name)
        } else {
            format!("{}@markPrice@1s", self.symbol.name)
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AllMarkPriceStream {
    is_second: bool,
}

impl AllMarkPriceStream {
    pub fn new(is_second: bool) -> AllMarkPriceStream {
        AllMarkPriceStream {  is_second }
    }
    pub fn is_second(&self) -> bool {
        self.is_second
    }
}


impl StreamNameFormat for AllMarkPriceStream {
    fn stream_name(&self) -> String {
        if self.is_second {
            "!markPrice@arr".to_string()
        } else {
            "!markPrice@arr@1s".to_string()
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct MarkPriceStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub mark_price: BigDecimal,
    #[serde(rename = "i")]
    pub index_price: BigDecimal,
    #[serde(rename = "P")]
    pub estimated_settle_price: BigDecimal,
    #[serde(rename = "r")]
    pub funding_rate: u64,
    #[serde(rename = "T")]
    pub next_funding_time: u64,
}
