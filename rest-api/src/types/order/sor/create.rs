use bigdecimal::BigDecimal;
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderResponseType, OrderSide, OrderType};
use general::error::SdkError;
use general::result::BinanceResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub struct CreateSorOrderReq {
    symbol: Option<String>,
    #[serde(rename = "side")]
    order_side: Option<OrderSide>,
    #[serde(rename = "type")]
    order_type: Option<OrderType>,
    #[serde(rename = "timeInForce")]
    time_in_force: Option<TimeInForce>,
    quantity: Option<BigDecimal>,
    price: Option<BigDecimal>,
    #[serde(rename = "newClientOrderId")]
    new_client_order_id: Option<String>,
    #[serde(rename = "strategyId")]
    strategy_id: Option<i64>,
    #[serde(rename = "strategyType")]
    strategy_type: Option<i32>,
    #[serde(rename = "icebergQty")]
    iceberg_qty: Option<BigDecimal>,
    #[serde(rename = "newOrderRespType")]
    new_order_resp_type: Option<OrderResponseType>,
    #[serde(rename = "selfTradePreventionMode")]
    self_trade_prevention_mode: Option<STPModel>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u16>,
}

pub struct CreateSorOrderReqBuilder {
    req: CreateSorOrderReq,
}

impl CreateSorOrderReqBuilder {
    pub fn new_builder() -> Self {
        Default::default()
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.req.symbol = Some(symbol.to_string());
        self
    }
    pub fn order_side(mut self, order_side: OrderSide) -> Self {
        self.req.order_side = Some(order_side);
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
    pub fn price(mut self, price: BigDecimal) -> Self {
        self.req.price = Some(price);
        self
    }
    pub fn new_client_order_id(mut self, new_client_order_id: String) -> Self {
        self.req.new_client_order_id = Some(new_client_order_id);
        self
    }
    pub fn strategy_type(mut self, strategy_type: i32) -> Self {
        self.req.strategy_type = Some(strategy_type);
        self
    }
    pub fn iceberg_qty(mut self, iceberg_qty: BigDecimal) -> Self {
        self.req.iceberg_qty = Some(iceberg_qty);
        self
    }
    pub fn new_order_resp_type(mut self, new_order_resp_type: OrderResponseType) -> Self {
        self.req.new_order_resp_type = Some(new_order_resp_type);
        self
    }
    pub fn self_trade_prevention_mode(mut self, stp_model: STPModel) -> Self {
        self.req.self_trade_prevention_mode = Some(stp_model);
        self
    }
    pub fn recv_window(mut self, recv_window: u16) -> Self {
        self.req.recv_window = Some(recv_window);
        self
    }

    pub fn build(self) -> BinanceResult<CreateSorOrderReq> {
        if self.req.symbol.is_none()
            || self.req.order_side.is_none()
            || self.req.order_type.is_none()
            || self.req.quantity.is_none()
        {
            return Err(SdkError::ParameterError(
                "sor order params: symbol, order_size, order_type, quantity must have value"
                    .to_string(),
            ));
        }
        Ok(self.req)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SorOrderFillResp {
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
pub struct CreateSorOrderResp {
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
