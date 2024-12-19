use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct UserDataStream {
    #[serde(rename="listenKey")]
    pub listen_key: String,
}

impl UserDataStream {
    pub fn new(listen_key: &str) -> Self {
        UserDataStream {
            listen_key: listen_key.to_string(),
        }
    }
}

impl StreamNameFormat for UserDataStream {
    fn stream_name(&self) -> String {
        self.listen_key.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct ListenKeyExpireEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "listenKey")]
    pub listen_key: String
}

#[derive(Debug, Deserialize)]
pub struct ExternalLockedEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "d")]
    pub delta: BigDecimal,
    #[serde(rename = "T")]
    pub transaction_time: u64
}