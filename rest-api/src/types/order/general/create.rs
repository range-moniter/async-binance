use bigdecimal::BigDecimal;
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide, OrderStatus, OrderType};
use general::error::SdkError;
use general::result::BinanceResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct CreateOrderReq {
    pub(super) symbol: Option<String>,
    side: Option<OrderSide>,
    #[serde(rename = "type")]
    order_type: Option<OrderType>,
    #[serde(rename = "timeInForce")]
    time_in_force: Option<TimeInForce>,
    #[serde(rename = "quantity")]
    quantity: Option<BigDecimal>,
    #[serde(rename = "quoteOrderQty")]
    quote_order_qty: Option<BigDecimal>,
    #[serde(rename = "price")]
    price: Option<BigDecimal>,
    #[serde(rename = "newClientOrderId")]
    new_client_order_id: Option<String>,
    #[serde(rename = "strategyId")]
    strategy_id: Option<u64>,
    #[serde(rename = "strategyType")]
    strategy_type: Option<u64>,
    #[serde(rename = "stopPrice")]
    stop_price: Option<BigDecimal>,
    #[serde(rename = "trailingDelta")]
    trailing_delta: Option<u64>,
    #[serde(rename = "icebergQty")]
    iceberg_qty: Option<BigDecimal>,
    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u64>,
}

pub struct CreateOrderReqBuilder {
    req: CreateOrderReq,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderFillResp {
    pub price: BigDecimal,
    pub qty: BigDecimal,
    pub commission: BigDecimal,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    #[serde(rename = "tradeId")]
    pub trade_id: u64,
}
#[derive(Debug, Deserialize)]
pub struct CreateOrderResp {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: u64,
    #[serde(rename = "orderListId")]
    pub order_list_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "transactTime")]
    pub transact_time: u64,
    #[serde(rename = "price")]
    pub price: Option<BigDecimal>,
    #[serde(rename = "origQty")]
    pub orig_qty: Option<BigDecimal>,
    #[serde(rename = "executedQty")]
    pub executed_qty: Option<BigDecimal>,
    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Option<BigDecimal>,
    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Option<BigDecimal>,
    #[serde(rename = "status")]
    pub status: Option<OrderStatus>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "side")]
    pub order_side: Option<OrderSide>,
    #[serde(rename = "workingTime")]
    pub working_time: Option<u64>,
    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename = "fills")]
    pub fills: Vec<CreateOrderFillResp>,
}

impl CreateOrderReqBuilder {
    pub fn new_builder() -> CreateOrderReqBuilder {
        CreateOrderReqBuilder {
            req: Default::default(),
        }
    }
    pub fn symbol(mut self, symbol: &str) -> Self {
        self.req.symbol = Some(symbol.to_string());
        self
    }
    pub fn side(mut self, side: OrderSide) -> Self {
        self.req.side = Some(side);
        self
    }
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.req.order_type = Some(order_type);
        self
    }
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.req.time_in_force = Some(time_in_force);
        self
    }
    pub fn quantity(mut self, quantity: BigDecimal) -> Self {
        self.req.quantity = Some(quantity);
        self
    }
    pub fn quote_order_qty(mut self, quote_order_qty: BigDecimal) -> Self {
        self.req.quote_order_qty = Some(quote_order_qty);
        self
    }
    pub fn price(mut self, price: BigDecimal) ->  Self {
        self.req.price = Some(price);
        self
    }
    pub fn new_client_order_id(mut self, new_client_order_id: &str) ->  Self {
        self.req.new_client_order_id = Some(new_client_order_id.to_string());
        self
    }
    pub fn strategy_id(mut self, strategy_id: u64) ->  Self {
        self.req.strategy_id = Some(strategy_id);
        self
    }
    pub fn strategy_type(mut self, strategy_type: u64) ->  Self {
        if strategy_type < 1000000 {
            panic!("strategy type must be greater than 1000000")
        }
        self.req.strategy_type = Some(strategy_type);
        self
    }
    pub fn stop_price(mut self, stop_price: BigDecimal) ->  Self {
        self.req.stop_price = Some(stop_price);
        self
    }
    pub fn trailing_delta(mut self, trailing_delta: u64) ->  Self {
        self.req.trailing_delta = Some(trailing_delta);
        self
    }
    pub fn iceberg_qty(mut self, iceberg_qty: BigDecimal) ->  Self {
        self.req.iceberg_qty = Some(iceberg_qty);
        self
    }
    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) ->  Self {
        self.req.new_order_resp_type = Some(new_order_resp_type);
        self
    }
    pub fn self_trade_prevention_mode(
        mut self,
        self_trade_prevention_mode: STPModel,
    ) ->  Self {
        self.req.self_trade_prevention_mode = Some(self_trade_prevention_mode);
        self
    }
    pub fn recv_window(mut self, recv_window: u64) ->  Self {
        self.req.recv_window = Some(recv_window);
        self
    }
    pub fn build(mut self) -> BinanceResult<CreateOrderReq> {
        if self.req.order_type.is_none() {
            return Err(SdkError::ParameterError("empty order type!".to_string()));
        }
        match self.req.order_type.unwrap().clone() {
            OrderType::LIMIT => {
                if self.req.quantity.is_none()
                    || self.req.time_in_force.is_none()
                    || self.req.price.is_none()
                {
                    return Err(SdkError::ParameterError(
                        self.error_format("LIMIT", "quantity,time_in_force,price"),
                    ));
                }
            }
            OrderType::MARKET => {
                if self.req.quantity.is_none() {
                    return Err(SdkError::ParameterError(
                        self.error_format("MARKET", "quantity"),
                    ));
                }
            }
            OrderType::StopLoss => {
                if self.req.stop_price.is_none()
                    || self.req.quantity.is_none()
                    || self.req.trailing_delta.is_none()
                {
                    return Err(SdkError::ParameterError(
                        self.error_format("STOP_LOSS", "stop_price,quantity,trailing_delta"),
                    ));
                }
            }
            OrderType::StopLossLimit | OrderType::TakeProfitLimit => {
                if self.req.time_in_force.is_none()
                    || self.req.quantity.is_none()
                    || self.req.price.is_none()
                    || self.req.stop_price.is_none()
                    || self.req.trailing_delta.is_none()
                {
                    return Err(SdkError::ParameterError(self.error_format(
                        "STOP_LOSS_LIMIT/TAKE_PROFIT_LIMIT",
                        "time_in_force,quantity,price,stop_price,trailing_delta",
                    )));
                }
            }
            OrderType::TakeProfit => {
                if self.req.quantity.is_none()
                    || self.req.stop_price.is_none()
                    || self.req.trailing_delta.is_none()
                {
                    return Err(SdkError::ParameterError(
                        self.error_format("TAKE_PROFIT", "quantity,stop_price,trailing_delta"),
                    ));
                }
            }
            OrderType::LimitMaker => {
                if self.req.quantity.is_none() || self.req.price.is_none() {
                    return Err(SdkError::ParameterError(
                        self.error_format("LIMIT_MAKER", "quantity,price"),
                    ));
                }
            }
        }
        if !self.req.iceberg_qty.is_none() && self.req.time_in_force.unwrap() != TimeInForce::GTC {
            self.req.time_in_force = Some(TimeInForce::GTC)
        }
        if !self.req.stop_price.is_none() {
            let order_type = self.req.order_type.unwrap();
            if order_type != OrderType::StopLoss
                && order_type != OrderType::StopLossLimit
                && order_type != OrderType::TakeProfit
                && order_type != OrderType::TakeProfitLimit
            {
                return Err(SdkError::ParameterError("when stopPrice have value, order type must be \
                            StopLoss or StopLossLimit or TakeProfit or TakeProfitLimit".to_string()));
            }
        }
        Ok(self.req)
    }

    fn error_format(&self, order_type: &str, params: &str) -> String {
        format!(
            "when order type is {}, this params must have value: {}",
            order_type, params
        )
    }
}
