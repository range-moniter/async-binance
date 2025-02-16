use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LookupTradeListReq {
    symbol: String,
    limit: u64,
    #[serde(rename = "formId")]
    form_id: Option<u64>,
}

impl LookupTradeListReq {
    pub fn new(symbol: &str) -> Self {
        LookupTradeListReq {
            symbol: symbol.to_string(),
            limit: 500,
            form_id: None,
        }
    }

    pub fn new_with_params(symbol: &str, limit: u64, form_id: Option<u64>) -> Self {
        let limit = if limit > 1000 { 1000 } else { limit };
        LookupTradeListReq {
            symbol: symbol.to_string(),
            limit,
            form_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AggTradeListReq {
    #[serde(rename = "symbol")]
    symbol: String,
    #[serde(rename = "fromId")]
    form_id: Option<u64>,
    #[serde(rename = "startTime")]
    start_time: Option<u64>,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    #[serde(rename = "limit")]
    limit: Option<u64>,
}

impl AggTradeListReq {
    pub fn new(symbol: &str) -> Self {
        AggTradeListReq {
            symbol: symbol.to_string(),
            form_id: None,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn set_form_id(&mut self, form_id: Option<u64>) {
        self.form_id = form_id;
    }
    pub fn set_start_time(&mut self, start_time: Option<u64>) {
        self.start_time = start_time;
    }
    pub fn set_end_time(&mut self, end_time: Option<u64>) {
        self.end_time = end_time;
    }
    pub fn set_limit(&mut self, limit: Option<u64>) {
        self.limit = limit;
    }
}

#[derive(Debug, Deserialize)]
pub struct TradeListResp {
    pub id: u64,
    pub price: BigDecimal,
    pub qty: BigDecimal,
    #[serde(rename = "quoteQty")]
    pub quote_qty: BigDecimal,
    pub time: u64,
    #[serde(rename = "isBuyerMaker")]
    pub is_buyer_maker: bool,
    #[serde(rename = "isBestMatch", default)]
    pub is_best_match: bool,
}

#[derive(Debug, Deserialize)]
pub struct AggTradeListResp {
    #[serde(rename = "a")]
    pub agg_trade_id: u64,
    #[serde(rename = "p")]
    pub price: BigDecimal,
    #[serde(rename = "q")]
    pub quantity: BigDecimal,
    #[serde(rename = "f")]
    pub first_trade_id: u64,
    #[serde(rename = "l")]
    pub last_trade_id: u64,
    #[serde(rename = "T")]
    pub trade_time: u128,
    #[serde(rename = "m")]
    pub is_buyer: bool,
    #[serde(rename = "M", default)]
    pub addition: bool,
}
