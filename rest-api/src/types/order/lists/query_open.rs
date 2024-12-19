use serde::{Deserialize, Serialize};
use crate::types::order::lists::OrderId;

#[derive(Serialize, Debug)]
pub struct QueryOpenOrderReq {
    #[serde(rename = "recvWindow")]
    recv_window: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct QueryOpenOrderResp {
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