use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::interval::Interval;

#[derive(Debug, Serialize)]
pub struct KlineReq {
    symbol: String,
    interval: Interval,
    #[serde(rename = "startTime")]
    start_time: Option<u64>,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    timezone: Option<String>,
    limit: Option<u8>,
}

impl KlineReq {
    pub fn new(symbol: &str, interval: Interval) -> Self {
        KlineReq {
            symbol: symbol.to_string(),
            interval,
            start_time: None,
            end_time: None,
            timezone: None,
            limit: None,
        }
    }

    pub fn set_start_time(mut self, start_time: u64) {
        self.start_time = Some(start_time);
    }
    pub fn set_end_time(mut self, end_time: u64) {
        self.end_time = Some(end_time);
    }
    pub fn set_timezone(mut self, timezone: String) {
        self.timezone = Some(timezone);
    }
    pub fn set_limit(mut self, limit: u8) {
        self.limit = Some(limit);
    }
}


#[derive(Debug, Deserialize)]
pub struct KlineTupleResp(u64, BigDecimal, BigDecimal, BigDecimal, BigDecimal, BigDecimal, u64, BigDecimal, u64, BigDecimal, BigDecimal, String);

#[derive(Debug)]
pub struct KlineResp {
    pub open_time: u64,
    pub close_time: u64,
    pub high_price: BigDecimal,
    pub low_price: BigDecimal,
    pub open_price: BigDecimal,
    pub close_price: BigDecimal,
    pub volume: BigDecimal,
    pub quote_asset_volume: BigDecimal,
    pub trade_count: u64,
    pub buyer_base_asset_volume: BigDecimal,
    pub buyer_quote_asset_volume: BigDecimal,
}

impl From<KlineTupleResp> for KlineResp {
    fn from(tuple: KlineTupleResp) -> Self {
        KlineResp {
            open_time: tuple.0,
            open_price: tuple.1,
            high_price: tuple.2,
            low_price: tuple.3,
            close_price: tuple.4,
            volume: tuple.5,
            close_time: tuple.6,
            quote_asset_volume: tuple.7,
            trade_count: tuple.8,
            buyer_base_asset_volume: tuple.9,
            buyer_quote_asset_volume: tuple.10,
        }
    }
}