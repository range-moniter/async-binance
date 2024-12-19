use bigdecimal::BigDecimal;
use serde::Deserialize;
use general::enums::general::{STPModel, TimeInForce};
use general::enums::order::{OrderSide, OrderStatus, OrderType};

#[derive(Debug, Deserialize)]
pub struct OrderEvent {
    #[serde(rename="E")]
    pub event_time: u64,
    #[serde(rename="s")]
    pub symbol: String,
    #[serde(rename="c")]
    pub client_order_id: String,
    #[serde(rename="S")]
    pub order_side: OrderSide,
    #[serde(rename="o")]
    pub order_type: OrderType,
    #[serde(rename="f")]
    pub time_in_force: TimeInForce,
    #[serde(rename="q")]
    pub quantity: BigDecimal,
    #[serde(rename="p")]
    pub price: BigDecimal,
    #[serde(rename="P")]
    pub stop_price: BigDecimal,
    #[serde(rename="F")]
    pub iceberg_qty: BigDecimal,
    #[serde(rename="g")]
    pub order_list_id: String,
    #[serde(rename="C")]
    pub orig_client_order_id: String,
    #[serde(rename="x")]
    pub execution_type: String,
    #[serde(rename="X")]
    pub order_status: OrderStatus,
    #[serde(rename="r")]
    pub reject_reason: String,
    #[serde(rename="i")]
    pub order_id: u64,
    #[serde(rename="l")]
    pub last_executed_qty: BigDecimal,
    #[serde(rename="z")]
    pub cumulative_filled_qty: BigDecimal,
    #[serde(rename="L")]
    pub last_executed_price: BigDecimal,
    #[serde(rename="n")]
    pub commission_amount: String,
    #[serde(rename="N")]
    pub commission_asset: Option<String>,
    #[serde(rename="T")]
    pub transaction_time: u64,
    #[serde(rename="t")]
    pub trade_id: i64,
    #[serde(rename="I")]
    pub ignore_i: i64,
    #[serde(rename="w")]
    pub is_on_book: bool,
    #[serde(rename="m")]
    pub is_maker_side: bool,
    #[serde(rename="M")]
    pub ignore_m: bool,
    #[serde(rename="O")]
    pub order_creation_time: u64,
    #[serde(rename="Z")]
    pub cumulative_quote_asset_transacted_qty: BigDecimal,
    #[serde(rename="Y")]
    pub last_quote_asset_transacted_qty: BigDecimal,
    #[serde(rename="Q")]
    pub quote_order_qty: BigDecimal,
    #[serde(rename="W")]
    pub working_time: u64,
    #[serde(rename="V")]
    pub self_trade_prevent_mode: Option<STPModel>,
    #[serde(rename="d")]
    pub trailing_delta: i64,
    #[serde(rename="D")]
    pub trailing_time:  u64,
    #[serde(rename="j")]
    pub strategy_id: i64,
    #[serde(rename="J")]
    pub strategy_type: i32,
    #[serde(rename="v")]
    pub prevented_match_id: i64,
    #[serde(rename="A")]
    pub prevented_qty: BigDecimal,
    #[serde(rename="B")]
    pub last_prevented_qty: BigDecimal,
    #[serde(rename="u")]
    pub trade_group_id: u64,
    #[serde(rename="U")]
    pub counter_order_id: u64,
    #[serde(rename="Cs")]
    pub counter_symbol: String,
    #[serde(rename="pl")]
    pub prevented_execution_qty: BigDecimal,
    #[serde(rename="pL")]
    pub prevented_execution_price: BigDecimal,
    #[serde(rename="pY")]
    pub prevented_execution_quote_qty: BigDecimal,
    #[serde(rename="b")]
    pub match_type: String,
    #[serde(rename="a")]
    pub working_floor: String,
    #[serde(rename="k")]
    pub allocation_id: u64,
    #[serde(rename="uS")]
    pub used_sor: bool,
}