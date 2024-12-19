use crate::types::order::lists::OrderId;
use bigdecimal::BigDecimal;
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide};
use general::error::SdkError;
use general::result::BinanceResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum AboveType {
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "LIMIT_MAKER")]
    LimitMarker,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
}

#[derive(Serialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BelowType {
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
}

#[derive(Debug, Serialize, Default)]
pub struct CreateOcoOrderReq {
    symbol: Option<String>,
    #[serde(rename = "listClientOrderId")]
    list_client_order_id: Option<String>,
    #[serde(rename = "side")]
    order_side: Option<OrderSide>,
    quantity: Option<f64>,
    #[serde(rename = "aboveType")]
    above_type: Option<AboveType>,
    #[serde(rename = "aboveClientOrderId")]
    above_client_order_id: Option<String>,
    #[serde(rename = "aboveIcebergQty")]
    above_iceberg_qty: Option<i64>,
    #[serde(rename = "abovePrice")]
    above_price: Option<BigDecimal>,
    #[serde(rename = "aboveStopPrice")]
    above_stop_price: Option<BigDecimal>,
    #[serde(rename = "aboveTrailingDelta")]
    above_trailing_delta: Option<i64>,
    #[serde(rename = "aboveTimeInForce")]
    above_time_in_force: Option<TimeInForce>,
    #[serde(rename = "aboveStrategyId")]
    above_strategy_id: Option<i64>,
    #[serde(rename = "aboveStrategyType")]
    above_strategy_type: Option<i32>,
    #[serde(rename = "belowType")]
    below_type: Option<BelowType>,
    #[serde(rename = "belowClientOrderId")]
    below_client_order_id: Option<String>,
    #[serde(rename = "belowIcebergQty")]
    below_iceberg_qty: Option<i64>,
    #[serde(rename = "belowPrice")]
    below_price: Option<BigDecimal>,
    #[serde(rename = "belowStopPrice")]
    below_stop_price: Option<BigDecimal>,
    #[serde(rename = "belowTrailingDelta")]
    below_trailing_delta: Option<i64>,
    #[serde(rename = "belowTimeInForce")]
    below_time_in_force: Option<TimeInForce>,
    #[serde(rename = "belowStrategyId")]
    below_strategy_id: Option<i64>,
    #[serde(rename = "belowStrategyType")]
    below_strategy_type: Option<i32>,
    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u16>,
}

pub struct CreateOcoOrderReqBuilder {
    req: CreateOcoOrderReq,
}

impl CreateOcoOrderReqBuilder {
    pub fn new_builder() -> CreateOcoOrderReqBuilder {
        CreateOcoOrderReqBuilder {
            req: Default::default(),
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.req.symbol = Some(symbol.to_string());
        self
    }
    pub fn list_client_order_id(mut self, list_client_order_id: &str) -> Self {
        self.req.list_client_order_id = Some(list_client_order_id.to_string());
        self
    }
    pub fn order_side(mut self, order_side: OrderSide) -> Self {
        self.req.order_side = Some(order_side);
        self
    }
    pub fn quantity(mut self, quantity: f64) -> Self {
        self.req.quantity = Some(quantity);
        self
    }
    pub fn above_type(mut self, above_type: AboveType) -> Self {
        self.req.above_type = Some(above_type);
        self
    }
    pub fn above_client_order_id(mut self, above_client_order_id: &str) -> Self {
        self.req.above_client_order_id = Some(above_client_order_id.to_string());
        self
    }
    pub fn above_iceberg_qty(mut self, above_iceberg_qty: i64) -> Self {
        self.req.above_iceberg_qty = Some(above_iceberg_qty);
        self
    }
    pub fn above_price(mut self, above_price: BigDecimal) -> Self {
        self.req.above_price = Some(above_price);
        self
    }
    pub fn above_stop_price(mut self, above_stop_price: BigDecimal) -> Self {
        self.req.above_stop_price = Some(above_stop_price);
        self
    }
    pub fn above_trailing_delta(mut self, above_trailing_delta: i64) -> Self {
        self.req.above_trailing_delta = Some(above_trailing_delta);
        self
    }
    pub fn above_strategy_id(mut self, above_strategy_id: i64) -> Self {
        self.req.above_strategy_id = Some(above_strategy_id);
        self
    }
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.req.above_time_in_force = Some(time_in_force);
        self
    }
    pub fn strategy_type(mut self, strategy_type: i32) -> Self {
        self.req.above_strategy_type = Some(strategy_type);
        self
    }
    pub fn below_type(mut self, below_type: BelowType) -> Self {
        self.req.below_type = Some(below_type);
        self
    }
    pub fn below_client_order_id(mut self, below_client_order_id: &str) -> Self {
        self.req.below_client_order_id = Some(below_client_order_id.to_string());
        self
    }
    pub fn below_iceberg_qty(mut self, below_iceberg_qty: i64) -> Self {
        self.req.below_iceberg_qty = Some(below_iceberg_qty);
        self
    }
    pub fn below_price(mut self, below_price: BigDecimal) -> Self {
        self.req.below_price = Some(below_price);
        self
    }
    pub fn below_stop_price(mut self, below_stop_price: BigDecimal) -> Self {
        self.req.below_stop_price = Some(below_stop_price);
        self
    }
    pub fn below_trailing_delta(mut self, below_trailing_delta: i64) -> Self {
        self.req.below_trailing_delta = Some(below_trailing_delta);
        self
    }
    pub fn below_strategy_id(mut self, below_strategy_id: i64) -> Self {
        self.req.below_strategy_id = Some(below_strategy_id);
        self
    }
    pub fn below_strategy_type(mut self, below_strategy_type: i32) -> Self {
        self.req.below_strategy_type = Some(below_strategy_type);
        self
    }
    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.req.new_order_resp_type = Some(new_order_resp_type);
        self
    }
    pub fn self_trade_prevention_mode(mut self, self_trade_prevention_mode: STPModel) -> Self {
        self.req.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }
    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.req.recv_window = Some(recv_window);
        self
    }
    pub fn build(mut self) -> BinanceResult<CreateOcoOrderReq> {
        if self.req.symbol.is_none()
            || self.req.order_side.is_none()
            || self.req.quantity.is_none()
            || self.req.above_type.is_none()
            || self.req.below_type.is_none()
        {
            return Err(SdkError::ParameterError(
                "oco order must have symbol,order_side,quantity,above_type,below_type".to_string(),
            ));
        }
        if !self.req.above_iceberg_qty.is_none()
            && matches!(self.req.above_time_in_force, Some(val) if val != TimeInForce::GTC)
        {
            return Err(SdkError::ParameterError(
                "when above_iceberg_qty have value, the above_time_in_force must be GTC"
                    .to_string(),
            ));
        }
        if !self.req.below_iceberg_qty.is_none()
            && matches!(self.req.below_time_in_force, Some(val) if val != TimeInForce::GTC)
        {
            return Err(SdkError::ParameterError(
                "when below_iceberg_qty have value, the below_time_in_force must be GTC"
                    .to_string(),
            ));
        }
        if self.req.above_type.unwrap() == AboveType::StopLossLimit
            || self.req.above_type.unwrap() == AboveType::TakeProfitLimit
        {
            if self.req.above_time_in_force.is_none() || self.req.below_time_in_force.is_none() {
                return Err(SdkError::ParameterError(
                    "when above_type is StopLossLimit or TakeProfitLimit, \
                                    the above_time_in_force or below_time_in_force must be set."
                        .to_string(),
                ));
            }
        }
        Ok(self.req)
    }
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
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,
    #[serde(rename = "workingTime")]
    pub working_time: i64,
    #[serde(rename = "icebergQty")]
    pub iceberg_qty: Option<String>,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOcoOrderResp {
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
