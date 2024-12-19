use serde::{Deserialize, Serialize};
use crate::types::order::lists::OrderId;

#[derive(Serialize, Debug)]
pub struct QueryOrderListReq {
    #[serde(rename="orderListId")]
    order_list_id: u64,
    #[serde(rename="origClientOrderId")]
    orig_client_order_id: Option<String>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOrderListResp {
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(rename = "contingencyType")]
    pub contingency_type: String,
    #[serde(rename = "listStatusType")]
    pub list_status_type: String,
    #[serde(rename = "listOrderStatus")]
    pub list_order_status: String,
    #[serde(rename = "listClientOrderId")]
    pub list_client_order_id: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: i64,
    pub symbol: String,
    pub orders: Vec<OrderId>,
}