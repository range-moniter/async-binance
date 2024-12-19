use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CommonReq {
    symbol: String,
    limit: u16
}

impl CommonReq {
    pub fn new(symbol: &str, limit: u16) -> Self {
        let limit = if limit <= 0 {
            100
        } else if limit > 5000 {
            5000
        } else {
            limit
        };
        CommonReq {
            symbol: symbol.to_string(),
            limit
        }
    }

    pub fn get_limit(&self) -> u16 {
        self.limit
    }
}


#[derive(Debug, Deserialize)]
pub struct OrderBookResp {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    #[serde(rename = "bids")]
    pub bids: Vec<Vec<String>>,
    #[serde(rename = "asks")]
    pub asks: Vec<Vec<String>>,
}
