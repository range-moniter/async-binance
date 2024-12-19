use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderSide, OrderStatus, OrderType};

#[derive(Debug, Serialize)]
pub struct QueryOrderReq {
    symbol: String,
    #[serde(rename="orderId")]
    order_id: Option<u64>,
    #[serde(rename="origClientOrderId")]
    orig_client_order_id: Option<String>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u64>,
}

impl QueryOrderReq {
    pub fn new_with_order_id(symbol: &str, order_id: u64) -> Self {
        QueryOrderReq {
            symbol: symbol.to_string(),
            order_id: Some(order_id),
            orig_client_order_id: None,
            recv_window: None,
        }
    }

    pub fn new_with_orig_client_order_id(symbol: &str, orig_client_order_id: String) -> Self {
        QueryOrderReq {
            symbol: symbol.to_string(),
            order_id: None,
            orig_client_order_id: Some(orig_client_order_id),
            recv_window: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct QueryAllOrderReq {
    symbol: String,
    #[serde(rename="orderId")]
    order_id: Option<u64>,
    #[serde(rename="startTime")]
    start_time: Option<u64>,
    #[serde(rename="endTime")]
    end_time: Option<u64>,
    limit: u16,
    #[serde(rename="recvWindow")]
    recv_window: Option<u64>,
}

impl QueryAllOrderReq {
    pub fn new(symbol: &str) -> Self {
        QueryAllOrderReq {
            symbol: symbol.to_string(),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: 500,
            recv_window: None,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct QueryOrderResp {
    pub symbol: String,
    #[serde(rename="orderId")]
    pub order_id: u64,
    #[serde(rename="orderListId")]
    pub order_list_id: i64,
    #[serde(rename="clientOrderId")]
    pub client_order_id: String,
    #[serde(rename="price")]
    pub price: BigDecimal,
    #[serde(rename="origQty")]
    pub orig_qty: BigDecimal,
    #[serde(rename="executedQty")]
    pub executed_qty: BigDecimal,
    #[serde(rename="cummulativeQuoteQty")]
    pub cummulative_quote_qty: BigDecimal,
    #[serde(rename="status")]
    pub status: OrderStatus,
    #[serde(rename="timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename="type")]
    pub order_type: OrderType,
    #[serde(rename="side")]
    pub side: OrderSide,
    #[serde(rename="stopPrice")]
    pub stop_price: BigDecimal,
    #[serde(rename="icebergQty")]
    pub iceberg_qty: BigDecimal,
    pub time: u64,
    #[serde(rename="updateTime")]
    pub update_time: u64,
    #[serde(rename="isWorking")]
    pub is_working: bool,
    #[serde(rename="workingTime")]
    pub working_time: u64,
    #[serde(rename="origQuoteOrderQty")]
    pub orig_quote_order_qty: BigDecimal,
    #[serde(rename="selfTradePreventionMode")]
    pub self_trade_prevention_mode: STPModel
}
