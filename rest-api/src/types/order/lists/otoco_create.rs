use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide};
use crate::types::order::lists::oco_create::{AboveType, BelowType};
use crate::types::order::lists::OrderId;
use crate::types::order::lists::oto_create::WorkingType;

#[derive(Serialize, Debug)]
pub struct CreateOtoCoOrderReq {

    symbol: String,
    #[serde(rename="listClientOrderId")]
    list_client_order_id: Option<String>,
    #[serde(rename="listClientOrderId")]
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
    #[serde(rename="pendingSide")]
    pending_side: OrderSide,
    #[serde(rename="pendingQuantity")]
    pending_quantity: BigDecimal,
    #[serde(rename="pendingAboveType")]
    pending_above_type: AboveType,
    #[serde(rename="pendingAboveClientOrderId")]
    pending_above_client_order_id: Option<String>,
    #[serde(rename="pendingAbovePrice")]
    pending_above_price: Option<BigDecimal>,
    #[serde(rename="pendingAboveStopPrice")]
    pending_above_stop_price: Option<BigDecimal>,
    #[serde(rename="pendingAboveTrailingDelta")]
    pending_above_trailing_delta: Option<BigDecimal>,
    #[serde(rename="pendingAboveIcebergQty")]
    pending_above_iceberg_qty: Option<BigDecimal>,
    #[serde(rename="pendingAboveTimeInForce")]
    pending_above_time_in_force: Option<TimeInForce>,
    #[serde(rename="pendingAboveStrategyId")]
    pending_above_strategy_id: Option<i64>,
    #[serde(rename="pendingAboveStrategyType")]
    pending_above_strategy_type: Option<i32>,
    #[serde(rename="pendingBelowType")]
    pending_below_type: BelowType,
    #[serde(rename="pendingBelowClientOrderId")]
    pending_below_client_order_id: Option<String>,
    #[serde(rename="pendingBelowPrice")]
    pending_below_price: Option<BigDecimal>,
    #[serde(rename="pendingBelowStopPrice")]
    pending_below_stop_price: Option<BigDecimal>,
    #[serde(rename="pendingBelowTrailingDelta")]
    pending_below_trailing_delta: Option<BigDecimal>,
    #[serde(rename="pendingBelowIcebergQty")]
    pending_below_iceberg_qty: Option<BigDecimal>,
    #[serde(rename="pendingBelowTimeInForce")]
    pending_below_time_in_force: Option<TimeInForce>,
    #[serde(rename="pendingBelowStrategyId")]
    pending_below_strategy_id: Option<i64>,
    #[serde(rename="pendingBelowStrategyType")]
    pending_below_strategy_type: Option<i32>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>
}




#[derive(Debug, Deserialize)]
pub struct OtoCoOrderResp {
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
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct CreateOtoCoOrderResp {
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
    pub order_reports: Vec<OtoCoOrderResp>,
}