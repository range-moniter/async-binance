use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderSide, OrderStatus, OrderType};

#[derive(Serialize, Debug, Deserialize)]
pub enum CancelRestriction {
    #[serde(rename = "ONLY_NEW")]
    OnlyNew,
    #[serde(rename = "ONLY_PARTIALLY_FILLED")]
    OnlyPartiallyFilled,
}

#[derive(Debug, Serialize)]
pub struct CancelOrderReq {
    symbol: String,
    #[serde(rename="orderId")]
    order_id: Option<u64>,
    #[serde(rename="origClientOrderId")]
    orig_client_order_id: Option<String>,
    #[serde(rename="newClientOrderId")]
    new_client_order_id: Option<String>,
    #[serde(rename="cancelRestriction")]
    cancel_restrictions: Option<CancelRestriction>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>,
}


impl CancelOrderReq {
    pub fn new_with_order_id(symbol: &str, order_id: u64) -> CancelOrderReq {
        CancelOrderReq {
            symbol: symbol.to_string(),
            order_id: Some(order_id),
            orig_client_order_id: None,
            new_client_order_id: None,
            cancel_restrictions: None,
            recv_window: None,
        }
    }

    pub fn new_with_orig_client_order_id(symbol: &str, orig_client_order_id: &str) -> CancelOrderReq {
        CancelOrderReq {
            symbol: symbol.to_string(),
            order_id: None,
            orig_client_order_id: Some(orig_client_order_id.to_string()),
            new_client_order_id: None,
            cancel_restrictions: None,
            recv_window: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelOrderResp {
    pub symbol: String,
    #[serde(rename="orderId")]
    pub order_id: u64,
    #[serde(rename="orderListId")]
    pub order_list_id: i64,
    #[serde(rename="origClientOrderId")]
    pub orig_client_order_id: String,
    #[serde(rename="clientOrderId")]
    pub client_order_id: String,
    #[serde(rename="transactTime")]
    pub transact_time: u64,
    pub price: BigDecimal,
    #[serde(rename="origQty")]
    pub orig_qty: BigDecimal,
    #[serde(rename="executedQty")]
    pub executed_qty: BigDecimal,
    #[serde(rename="origQuoteOrderQty")]
    pub orig_quote_order_qty: BigDecimal,
    #[serde(rename="cummulativeQuoteQty")]
    pub cummulative_quote_qty: BigDecimal,
    #[serde(rename="status")]
    pub order_status: OrderStatus,
    #[serde(rename="timeInForce")]
    pub time_in_force: TimeInForce,
    #[serde(rename="type")]
    pub order_type: OrderType,
    #[serde(rename="side")]
    pub order_side: OrderSide,
    #[serde(rename="selfTradePreventionMode")]
    pub self_trade_prevention_mode: STPModel
}

#[derive(Debug, Serialize)]
pub struct CancelSingleTypeOrderReq{
    symbol: String,
    #[serde(rename="recvWindow")]
    recv_window: Option<u16>
}