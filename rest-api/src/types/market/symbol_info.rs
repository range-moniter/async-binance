use serde::{Deserialize, Serialize};
use general::enums::general::{FutureTimeInForce, TimeInForce};
use general::enums::order::{FutureOrderType, OrderType};
use general::enums::permission::Permission;
use general::enums::symbol_filter::{FutureSymbolFilter, SymbolFilter};
use general::enums::symbol_status::SymbolStatus;

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    symbol: String,
    status: SymbolStatus,
    #[serde(rename="baseAsset")]
    base_asset: String,
    #[serde(rename="baseAssetPrecision")]
    base_asset_precision: u8,
    #[serde(rename="quoteAsset")]
    quote_asset: String,
    #[serde(rename="quotePrecision")]
    quote_precision: u8,
    #[serde(rename="quoteAssetPrecision")]
    quote_asset_precision: u8,
    #[serde(rename="baseCommissionPrecision")]
    base_commission_precision: u8,
    #[serde(rename="quoteCommissionPrecision")]
    quote_commission_precision: u8,
    #[serde(rename="orderTypes")]
    order_types: Vec<OrderType>,
    #[serde(rename="icebergAllowed")]
    iceberg_allowed: bool,
    #[serde(rename="ocoAllowed")]
    oco_allowed: bool,
    #[serde(rename="otoAllowed")]
    oto_allowed: bool,
    #[serde(rename="quoteOrderQtyMarketAllowed")]
    quote_order_qty_market_allowed: bool,
    #[serde(rename="allowTrailingStop")]
    allow_trailing_stop: bool,
    #[serde(rename="cancelReplaceAllowed")]
    cancel_replace_allowed: bool,
    #[serde(rename="isSpotTradingAllowed")]
    is_spot_trading_allowed: bool,
    #[serde(rename="isMarginTradingAllowed")]
    is_margin_trading_allowed: bool,
    filters: Vec<SymbolFilter>,
    permissions: Vec<String>,
    #[serde(rename="permissionSets")]
    permission_sets: Vec<Vec<Permission>>,
    #[serde(rename="defaultSelfTradePreventionMode")]
    default_self_trade_prevention_mode: String,
    #[serde(rename="allowedSelfTradePreventionModes")]
    allowed_self_trade_prevention_modes: Vec<String>,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct FutureSymbol {
    pub symbol: String,
    pub pair: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    #[serde(rename = "deliveryDate")]
    pub delivery_date: i64,
    #[serde(rename = "onboardDate")]
    pub onboard_date: i64,
    pub status: String,
    #[serde(rename = "maintMarginPercent")]
    pub maint_margin_percent: String,
    #[serde(rename = "requiredMarginPercent")]
    pub required_margin_percent: String,
    #[serde(rename = "baseAsset")]
    pub base_asset: String,
    #[serde(rename = "quoteAsset")]
    pub quote_asset: String,
    #[serde(rename = "marginAsset")]
    pub margin_asset: String,
    #[serde(rename = "pricePrecision")]
    pub price_precision: i64,
    #[serde(rename = "quantityPrecision")]
    pub quantity_precision: i64,
    #[serde(rename = "baseAssetPrecision")]
    pub base_asset_precision: i64,
    #[serde(rename = "quotePrecision")]
    pub quote_precision: i64,
    #[serde(rename = "underlyingType")]
    pub underlying_type: String,
    #[serde(rename = "underlyingSubType")]
    pub underlying_sub_type: Vec<String>,
    #[serde(rename = "settlePlan")]
    pub settle_plan: Option<i64>,
    #[serde(rename = "triggerProtect")]
    pub trigger_protect: String,
    pub filters: Vec<FutureSymbolFilter>,
    #[serde(rename = "orderTypes")]
    pub order_types: Vec<FutureOrderType>,
    #[serde(rename = "timeInForce")]
    pub time_in_force: Vec<FutureTimeInForce>,
    #[serde(rename = "liquidationFee")]
    pub liquidation_fee: String,
    #[serde(rename = "marketTakeBound")]
    pub market_take_bound: String,
}
