use serde::Deserialize;
use general::enums::order::OrderType;
use general::enums::permission::Permission;
use general::enums::symbol_filter::SymbolFilter;
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


