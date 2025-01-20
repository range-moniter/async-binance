use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use client::stream::stream::StreamNameFormat;
use general::enums::contract_status::ContractStatus;
use general::enums::contract_type::ContractType;
use crate::market::types::event_type::EventType;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default)]
pub struct ContractInfoStream;

impl StreamNameFormat for ContractInfoStream {
    fn stream_name(&self) -> String {
        "!contractInfo".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractInfoStreamPayload {
    #[serde(rename = "e")]
    pub event_type: EventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "ps")]
    pub pair: String,
    #[serde(rename = "ct")]
    pub contract_type: ContractType,
    #[serde(rename = "dt")]
    pub delivery_date_time: u64,
    #[serde(rename = "ot")]
    pub onboard_date_time: u64,
    #[serde(rename = "cs")]
    pub contract_status: ContractStatus,
    #[serde(rename = "bks")]
    pub bks: Vec<Bks>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bks {
    #[serde(rename = "bs")]
    pub bracket: u64,
    #[serde(rename = "bnf")]
    pub floor: u64,
    #[serde(rename = "bnc")]
    pub cap: u64,
    #[serde(rename = "mmr")]
    pub maintenance_ratio: BigDecimal,
    #[serde(rename = "cf")]
    pub auxiliary_number: BigDecimal,
    #[serde(rename = "mi")]
    pub min_leverage: BigDecimal,
    #[serde(rename = "ma")]
    pub max_leverage: BigDecimal
}