use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy , Eq, PartialEq, Hash)]
pub enum ContractType {
    #[serde(rename = "perpetual")]
    Perpetual,
    #[serde(rename = "current_quarter")]
    CurrentQuarter,
    #[serde(rename = "next_quarter")]
    NextQuarter,
}