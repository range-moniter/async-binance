use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::enums::level::Level;
use general::enums::speed::Speed;
use general::symbol::Symbol;


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BookDepthStream {
    symbol: Symbol,
    level: Level,
    speed: Option<Speed>,
}

impl BookDepthStream {
    pub fn new(symbol: Symbol, level: Level, speed: Option<Speed>) -> BookDepthStream {
        BookDepthStream { symbol, level, speed }
    }
    pub fn get_level(&self) -> Level {
        self.level
    }
    pub fn get_symbol(&self) -> Symbol {
        self.symbol.clone()
    }
    pub fn get_speed(&self) -> Option<Speed> {
        self.speed
    }
}

impl StreamNameFormat for BookDepthStream {
    fn stream_name(&self) -> String {
        match &self.speed {
            None => format!("{}@depth{}", self.symbol.name, self.level.value()),
            Some(speed) => format!("{}@depth{}@{}", self.symbol.name, self.level.value(), speed.as_str())
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BookDepthStreamPayload {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    #[serde(rename = "bids")]
    bids: Vec<Vec<String>>,
    #[serde(rename = "asks")]
    asks: Vec<Vec<String>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        let mut stream = BookDepthStream::new(Symbol::new("S"), Level::L1);
        stream.put_speed(Speed::Ms1000);
        println!("{}", stream.stream_name());
    }

    #[test]
    fn test_payload_deserialize() {
        let payload = r#"
            {
                "lastUpdateId": 160,
                "bids": [
                    [
                        "0.0024",
                        "10"
                    ]
                ],
                "asks": [
                    [
                        "0.0026",
                        "100"
                    ]
                ]
            }
        "#;
        let book_payload = serde_json::from_str::<BookDepthStreamPayload>(payload).unwrap();
        print!("{:?}", book_payload);
    }
}