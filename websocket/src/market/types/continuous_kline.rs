use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::enums::contract_type::ContractType;
use general::enums::interval::Interval;
use general::symbol::Symbol;
use crate::market::types::event_type::EventType;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ContinuousKlineStream {
    symbol: Symbol,
    contract_type: ContractType,
    interval: Interval,
}

impl ContinuousKlineStream {
    pub fn new(symbol: Symbol, contract_type: ContractType, interval: Interval) -> Self {
        ContinuousKlineStream {
            symbol, contract_type, interval,
        }
    }

    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
    pub fn get_contract_type(&self) -> ContractType {
        self.contract_type
    }
    pub fn get_interval(&self) -> Interval {
        self.interval
    }
}

impl StreamNameFormat for ContinuousKlineStream {
    fn stream_name(&self) -> String {
        format!("{}_{}@continuousKline_{}", self.symbol.name, self.contract_type.as_str(), self.interval.as_str())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuousKlineStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "ps")]
    pub symbol: String,
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    #[serde(rename = "k")]
    pub body: ContinuousKlineBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContinuousKlineBody {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "i")]
    pub interval: Interval,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "o")]
    pub open_price: BigDecimal,
    #[serde(rename = "c")]
    pub close_price: BigDecimal,
    #[serde(rename = "h")]
    pub high_price: BigDecimal,
    #[serde(rename = "l")]
    pub low_price: BigDecimal,
    #[serde(rename = "v")]
    pub asset_volume: BigDecimal,
    #[serde(rename = "n")]
    pub trades_number: u64,
    #[serde(rename = "x")]
    pub is_kline_close: bool,
    #[serde(rename = "q")]
    pub quote_asset_volume: BigDecimal,
    #[serde(rename = "V")]
    pub taker_buy_base_asset_volume: BigDecimal,
    #[serde(rename = "Q")]
    pub taker_buy_quote_asset_volume: BigDecimal,
    #[serde(rename = "B")]
    pub addition: String,
}



