use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use crate::market::types::event_type::EventType;
use general::enums::speed::Speed;
use general::symbol::Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct DepthStream {
    symbol: Symbol,
    speed: Option<Speed>,
}


impl DepthStream {
    pub fn new(symbol: Symbol, speed: Option<Speed>) -> Self {
        DepthStream {symbol, speed }
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
    pub fn get_speed(&self) -> Option<Speed> {
        self.speed
    }
}

impl StreamNameFormat for DepthStream {
    fn stream_name(&self) -> String {
        match &self.speed {
            None => format!("{}@depth", self.symbol.name),
            Some(speed) => format!("{}@depth@{}", self.symbol.name, speed.as_str())
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DepthStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u32,
    #[serde(rename = "u")]
    pub final_update_id: u32,
    #[serde(rename = "b")]
    pub bids: Vec<Vec<String>>,
    #[serde(rename = "a")]
    pub asks: Vec<Vec<String>>,
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize() {
        let data = r#"
                {
                    "e": "depthUpdate",
                    "E": 1672515782136,
                    "s": "BNBBTC",
                    "U": 157,
                    "u": 160,
                    "b": [
                      [
                        "0.0024",
                        "10"
                      ]
                    ],
                    "a": [
                      [
                        "0.0026",
                        "100"
                      ]
                    ]
                }
        "#;
        let data = serde_json::from_str::<DepthStreamPayload>(&data).unwrap();
        println!("{:#?}", data);
    }
}