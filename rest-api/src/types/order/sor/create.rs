use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide, OrderType};

#[derive(Serialize, Debug)]
pub struct CreateSorOrderReq {
    symbol: String,
    #[serde(rename = "side")]
    order_side: OrderSide,
    #[serde(rename = "type")]
    order_type: OrderType,
    #[serde(rename = "timeInForce")]
    time_in_force: Option<TimeInForce>,
    quantity: BigDecimal,
    price: Option<BigDecimal>,
    #[serde(rename = "newClientOrderId")]
    new_client_order_id: Option<String>,
    #[serde(rename = "strategyId")]
    strategy_id: i64,
    #[serde(rename = "strategyType")]
    strategy_type: i32,
    #[serde(rename = "icebergQty")]
    iceberg_qty: BigDecimal,
    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct SorOrderFillResp {
    #[serde(rename = "matchType")]
    pub match_type: String,
    pub price: String,
    pub qty: String,
    pub commission: String,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
    #[serde(rename = "allocId")]
    pub alloc_id: i64,
}

#[derive(Serialize, Deserialize)]
struct CreateSorOrderResp {
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
    pub fills: Vec<SorOrderFillResp>,
    #[serde(rename = "workingFloor")]
    pub working_floor: String,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
    #[serde(rename = "usedSor")]
    pub used_sor: bool,
}