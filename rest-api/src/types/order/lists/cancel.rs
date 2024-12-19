use serde::{Deserialize, Serialize};
use crate::types::order::lists::OrderId;

#[derive(Debug, Serialize)]
pub struct CancelOrderListReq {
    symbol: String,
    #[serde(rename="orderListId")]
    order_list_id: Option<i64>,
    #[serde(rename="listClientOrderId")]
    list_client_order_id: Option<String>,
    #[serde(rename="newClientOrderId")]
    new_client_order_id: Option<String>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>
}


#[derive(Debug, Deserialize)]
pub struct OrderListResp {
    pub symbol: String,
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
    pub price: String,
    #[serde(rename = "origQty")]
    pub orig_qty: String,
    #[serde(rename = "executedQty")]
    pub executed_qty: String,
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: String,
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: String,
    pub status: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub side: String,
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderListResp {
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
    #[serde(rename = "orderReports")]
    pub order_reports: Vec<OrderListResp>,
}