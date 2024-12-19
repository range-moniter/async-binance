use serde::Deserialize;
use general::enums::general::ContingencyType;
use general::enums::order::{OrderListOrderStatus, OrderListStatus};

#[derive(Debug, Deserialize)]
pub struct OrderListEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    pub order_list_id: u64,
    #[serde(rename = "c")]
    pub contingency_type: ContingencyType,
    #[serde(rename = "l")]
    pub list_status: OrderListStatus,
    #[serde(rename = "L")]
    pub order_status: OrderListOrderStatus,
    #[serde(rename = "r")]
    pub reject_reason: Option<String>,
    #[serde(rename = "C")]
    pub client_order_id: String,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "O")]
    pub symbols: Vec<OrderId>,
}

#[derive(Debug, Deserialize)]
pub struct OrderId {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}

