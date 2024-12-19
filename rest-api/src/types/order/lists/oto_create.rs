use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide, OrderType};
use crate::types::order::lists::OrderId;

#[derive(Serialize, Debug)]
pub enum WorkingType {
    #[serde(rename="LIMIT")]
    Limit,
    #[serde(rename="LIMIT_MAKER")]
    LimitMaker,
}

#[derive(Serialize, Debug)]
pub struct CreateOtoOrderReq {
    symbol: String,
    #[serde(rename="listClientOrderId")]
    list_client_order_id: Option<String>,
    #[serde(rename="newOrderRespType")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(rename="selfTradePreventionMode")]
    self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename="workingType")]
    working_type: WorkingType,
    #[serde(rename="workingSide")]
    working_side: OrderSide,
    #[serde(rename="workingClientOrderId")]
    working_client_order_id: Option<String>,
    #[serde(rename="workingPrice")]
    working_price: BigDecimal,
    #[serde(rename="workingQuantity")]
    working_quantity: BigDecimal,
    #[serde(rename="workingIcebergQty")]
    working_iceberg_qty: Option<BigDecimal>,
    #[serde(rename="workingTimeInForce")]
    working_time_in_force: Option<TimeInForce>,
    #[serde(rename="workingStrategyId")]
    working_strategy_id: Option<i64>,
    #[serde(rename="workingStrategyType")]
    working_strategy_type: Option<i32>,
    #[serde(rename="pendingType")]
    pending_type: OrderType,
    #[serde(rename="pendingSide")]
    pending_side: OrderSide,
    #[serde(rename="pendingClientOrderId")]
    pending_client_order_id: Option<String>,
    #[serde(rename="pendingPrice")]
    pending_price: Option<BigDecimal>,
    #[serde(rename="pendingStopPrice")]
    pending_stop_price: Option<BigDecimal>,
    #[serde(rename="pendingTrailingDelta")]
    pending_trailing_delta: Option<BigDecimal>,
    #[serde(rename="pendingQuantity")]
    pending_quantity: BigDecimal,
    #[serde(rename="pendingIcebergQty")]
    pending_iceberg_qty: Option<BigDecimal>,
    #[serde(rename="pendingTimeInForce")]
    pending_time_in_force: Option<TimeInForce>,
    #[serde(rename="pendingStrategyId")]
    pending_strategy_id: Option<i64>,
    #[serde(rename="pendingStrategyType")]
    pending_strategy_type: Option<i32>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>,
}


#[derive(Debug, Deserialize)]
pub struct OcoOrderResp {
    pub symbol: String,
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
    #[serde(rename = "workingTime")]
    pub working_time: i64,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}


#[derive(Debug, Deserialize)]
pub struct CreateOtoOrderResp {
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
    pub order_reports: Vec<OcoOrderResp>,
}