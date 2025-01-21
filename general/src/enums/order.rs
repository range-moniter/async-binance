use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OrderStatus {
    NEW,
    #[serde(rename="PENDING_NEW")]
    PendingNew,
    #[serde(rename="PARTIALLY_FILLED")]
    PartiallyFilled,
    FILLED,
    CANCELED,
    #[serde(rename="PENDING_CANCEL")]
    PendingCancel,
    REJECTED,
    EXPIRED,
    #[serde(rename="EXPIRED_IN_MATCH")]
    ExpiredMatch
}
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OrderListStatus {
    RESPONSE,
    #[serde(rename="EXEC_STARTED")]
    ExecStarted,
    #[serde(rename="ALL_DONE")]
    AllDone
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OrderListOrderStatus {
    EXECUTING,
    #[serde(rename="ALL_DONE")]
    AllDone,
    REJECT
}
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum OrderType {
    LIMIT,
    MARKET,
    #[serde(rename="STOP_LOSS")]
    StopLoss,
    #[serde(rename="STOP_LOSS_LIMIT")]
    StopLossLimit,
    #[serde(rename="TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename="TAKE_PROFIT_LIMIT")]
    TakeProfitLimit,
    #[serde(rename="LIMIT_MAKER")]
    LimitMaker
}
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum FutureOrderType {
    #[serde(rename="LIMIT")]
    Limit,
    #[serde(rename="MARKET")]
    Market,
    #[serde(rename="STOP")]
    Stop,
    #[serde(rename="STOP_MARKET")]
    StopMarket,
    #[serde(rename="TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename="TAKE_PROFIT_MARKET")]
    TakeProfitMarket,
    #[serde(rename="TRAILING_STOP_MARKET")]
    TrailingStopMarket,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OrderResponseType {
    ACK,
    RESULT,
    FULL
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum OrderSide {
    BUY,
    SELL,
}