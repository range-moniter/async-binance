use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::query_type::QueryType;
use general::serialize_extend::serialize_option_vec;

#[derive(Clone, Serialize, Debug)]
pub struct AvgPriceReq {
    symbol: String,
}


impl AvgPriceReq {
    pub fn new(symbol: &str) -> Self {
        AvgPriceReq {
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AvgPriceResp {
    pub mins: u8,
    pub price: BigDecimal,
    #[serde(rename = "closeTime")]
    pub close_time: u64,
}


#[derive(Debug, Serialize)]
pub struct PriceChangeTickerReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<String>,
    #[serde(serialize_with = "serialize_option_vec", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    types: Option<QueryType>,
}

impl PriceChangeTickerReq {
    pub fn new_with_single(symbol: &str) -> Self {
        PriceChangeTickerReq {
            symbol: Some(symbol.to_string()),
            symbols: None,
            types: None,
        }
    }
    pub fn new_with_multiple(symbols: Vec<&str>) -> Self {
        PriceChangeTickerReq {
            symbols: Some(symbols.into_iter().map(|item| item.to_string()).collect::<Vec<_>>()),
            symbol: None,
            types: None,
        }
    }
    pub fn set_types(&mut self, types: QueryType) {
        self.types = Some(types);
    }
    pub fn symbols_len(&self) -> usize {
        self.symbols.as_ref().map(|s| s.len()).unwrap_or(0)
    }
    pub fn symbols_is_none(&self) -> bool {
        self.symbols.is_none()
    }
    pub fn symbol_is_none(&self) -> bool {
        self.symbol.is_none()
    }
}


#[derive(Debug, Deserialize)]
pub struct PriceChangeTickerStatResp {
    pub symbol: String,
    #[serde(rename = "priceChange")]
    pub price_change: Option<BigDecimal>,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<BigDecimal>,
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<BigDecimal>,
    #[serde(rename = "prevClosePrice")]
    pub prev_close_price: Option<BigDecimal>,
    #[serde(rename = "lastPrice")]
    pub last_price: BigDecimal,
    #[serde(rename = "lastQty")]
    pub last_qty: Option<BigDecimal>,
    #[serde(rename = "bidPrice")]
    pub bid_price: Option<BigDecimal>,
    #[serde(rename = "bidQty")]
    pub bid_qty: Option<BigDecimal>,
    #[serde(rename = "askPrice")]
    pub ask_price: Option<BigDecimal>,
    #[serde(rename = "askQty")]
    pub ask_qty: Option<BigDecimal>,
    #[serde(rename = "openPrice")]
    pub open_price: BigDecimal,
    #[serde(rename = "highPrice")]
    pub high_price: BigDecimal,
    #[serde(rename = "lowPrice")]
    pub low_price: BigDecimal,
    pub volume: Option<BigDecimal>,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: BigDecimal,
    #[serde(rename = "openTime")]
    pub open_time: u64,
    #[serde(rename = "closeTime")]
    pub close_time: u64,
    #[serde(rename = "firstId")]
    pub first_id: u64,
    #[serde(rename = "lastId")]
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct PriceTradeDayTickerReq {
    symbol: Option<String>,
    #[serde(serialize_with = "serialize_option_vec", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "timeZone")]
    timezone: Option<String>,
    #[serde(rename = "type")]
    types: Option<QueryType>,
}

impl PriceTradeDayTickerReq {
    pub fn new_with_single(symbol: &str) -> Self {
        PriceTradeDayTickerReq {
            symbol: Some(symbol.to_string()),
            symbols: None,
            timezone: None,
            types: None,
        }
    }

    pub fn new_with_multiple(symbols: Vec<&str>) -> Self {
        PriceTradeDayTickerReq {
            symbols: Some(symbols.into_iter().map(|item| item.to_string()).collect::<Vec<_>>()),
            symbol: None,
            timezone: None,
            types: None,
        }
    }
    pub fn set_types(&mut self, types: QueryType) {
        self.types = Some(types);
    }
    pub fn set_timezone(&mut self, timezone: String) {
        self.timezone = Some(timezone);
    }
    pub fn symbols_is_none(&self) -> bool {
        self.symbols.is_none()
    }
    pub fn symbol_is_none(&self) -> bool {
        self.symbol.is_none()
    }

    pub fn get_symbols_len(&self) -> usize {
        self.symbols.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}


#[derive(Debug, Deserialize)]
pub struct PriceTradeDayTickerResp {
    pub symbol: String,
    #[serde(rename = "priceChange")]
    pub price_change: Option<BigDecimal>,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<BigDecimal>,
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<BigDecimal>,
    #[serde(rename = "openPrice")]
    pub open_price: BigDecimal,
    #[serde(rename = "highPrice")]
    pub high_price: BigDecimal,
    #[serde(rename = "lowPrice")]
    pub low_price: BigDecimal,
    #[serde(rename = "lastPrice")]
    pub last_price: BigDecimal,
    #[serde(rename = "volume")]
    pub volume: BigDecimal,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: BigDecimal,
    #[serde(rename = "openTime")]
    pub open_time: u64,
    #[serde(rename = "closeTime")]
    pub close_time: u64,
    #[serde(rename = "firstId")]
    pub first_id: u64,
    #[serde(rename = "lastId")]
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct SymbolReq {
    symbol: Option<String>,
    #[serde(serialize_with = "serialize_option_vec", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
}

impl SymbolReq {
    pub fn new_with_single(symbol: &str) -> Self {
        SymbolReq {
            symbol: Some(symbol.to_string()),
            symbols: None,
        }
    }

    pub fn new_with_multiple(symbols: Vec<&str>) -> Self {
        SymbolReq {
            symbol: None,
            symbols: Some(symbols.into_iter().map(|item| item.to_string()).collect::<Vec<_>>()),
        }
    }
    pub fn symbols_is_none(&self) -> bool {
        self.symbols.is_none()
    }
    pub fn symbol_is_none(&self) -> bool {
        self.symbol.is_none()
    }
}

#[derive(Debug, Deserialize)]
pub struct SymbolPriceTickerResp {
    pub symbol: String,
    pub price: BigDecimal,
}


#[derive(Debug, Deserialize)]
pub struct SymbolOrderBookResp {
    pub symbol: String,
    #[serde(rename = "bidPrice")]
    pub bid_price: BigDecimal,
    #[serde(rename = "bidQty")]
    pub bid_qty: BigDecimal,
    #[serde(rename = "askPrice")]
    pub ask_price: BigDecimal,
    #[serde(rename = "askQty")]
    pub ask_qty: BigDecimal,
}


#[derive(Debug, Serialize)]
pub struct RollingWindowPriceChangeStatReq {
    symbol: Option<String>,
    #[serde(serialize_with = "serialize_option_vec", skip_serializing_if = "Option::is_none")]
    symbols: Option<Vec<String>>,
    #[serde(rename = "type")]
    types: Option<QueryType>,
    window_size: Option<String>
}

impl RollingWindowPriceChangeStatReq {

    pub fn new_with_single(symbol: &str) -> Self {
        RollingWindowPriceChangeStatReq {
            symbol: Some(symbol.to_string()),
            symbols: None,
            types: None,
            window_size: None,
        }
    }

    pub fn new_with_multiple(symbols: Vec<&str>) -> Self {
        RollingWindowPriceChangeStatReq {
            symbol: None,
            symbols: Some(symbols.into_iter().map(|item|item.to_string()).collect::<Vec<_>>()),
            types: None,
            window_size: None,
        }
    }
    pub fn symbols_is_none(&self) -> bool {
        self.symbols.is_none()
    }
    pub fn symbol_is_none(&self) -> bool {
        self.symbol.is_none()
    }
    pub fn set_types(&mut self, types: QueryType) {
        self.types = Some(types);
    }
    pub fn get_symbols_len(&self) -> usize {
        self.symbols.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}


#[derive(Debug, Deserialize)]
pub struct RollingWindowPriceChangeStatResp {
    pub symbol: String,
    #[serde(rename = "priceChange")]
    pub price_change: Option<BigDecimal>,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: Option<BigDecimal>,
    #[serde(rename = "weightedAvgPrice")]
    pub weighted_avg_price: Option<BigDecimal>,
    #[serde(rename = "openPrice")]
    pub open_price: BigDecimal,
    #[serde(rename = "highPrice")]
    pub high_price: BigDecimal,
    #[serde(rename = "lowPrice")]
    pub low_price: BigDecimal,
    #[serde(rename = "lastPrice")]
    pub last_price: BigDecimal,
    #[serde(rename = "volume")]
    pub volume: BigDecimal,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: BigDecimal,
    #[serde(rename = "openTime")]
    pub open_time: u64,
    #[serde(rename = "closeTime")]
    pub close_time: u64,
    #[serde(rename = "firstId")]
    pub first_id: u64,
    #[serde(rename = "lastId")]
    pub last_id: u64,
    pub count: u64,
}