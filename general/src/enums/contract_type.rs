use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy , Eq, PartialEq, Hash)]
pub enum ContractType {
    #[serde(rename = "PERPETUAL")]
    Perpetual,
    #[serde(rename = "CURRENT_QUARTER")]
    CurrentQuarter,
    #[serde(rename = "NEXT_QUARTER")]
    NextQuarter,
    #[serde(rename = "CURRENT_MONTH")]
    CurrentMonth,
    #[serde(rename = "NEXT_MONTH")]
    NextMonth,
    #[serde(rename = "PERPETUAL_DELIVERING")]
    PerpetualDelivering,
}

impl ContractType {
    pub fn as_str(&self) -> &str {
        match self {
            ContractType::Perpetual => "perpetual",
            ContractType::CurrentQuarter => "current_quarter",
            ContractType::NextQuarter => "next_quarter",
            ContractType::CurrentMonth => "current_month",
            ContractType::NextMonth => "next_month",
            ContractType::PerpetualDelivering => "perpetual_delivering",
        }
    }
}