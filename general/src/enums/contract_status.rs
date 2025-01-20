use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy , Eq, PartialEq, Hash)]
pub enum ContractStatus {
    #[serde(rename = "PENDING_TRADING")]
    PendingTrading,
    #[serde(rename = "TRADING")]
    Trading,
    #[serde(rename = "PRE_DELIVERING")]
    PreDelivering,
    #[serde(rename = "DELIVERING")]
    Delivering,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "PRE_SETTLE")]
    PreSettle,
    #[serde(rename = "SETTLING")]
    Settling,
    #[serde(rename = "CLOSE")]
    Close
}