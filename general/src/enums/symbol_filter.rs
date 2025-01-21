use serde::{Deserialize, Serialize};
use crate::serialize_extend::from_str_to_f64;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "filterType")]
pub enum SymbolFilter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(rename = "minPrice", deserialize_with = "from_str_to_f64")]
        min_price: f64,
        #[serde(rename = "maxPrice", deserialize_with = "from_str_to_f64")]
        max_price: f64,
        #[serde(rename = "tickSize", deserialize_with = "from_str_to_f64")]
        tick_size: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(rename = "multiplierUp", deserialize_with = "from_str_to_f64")]
        multiplier_up: f64,
        #[serde(rename = "multiplierDown", deserialize_with = "from_str_to_f64")]
        multiplier_down: f64,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: i32,
    },
    #[serde(rename = "PERCENT_PRICE_BY_SIDE")]
    PercentPriceBySide {
        #[serde(rename = "bidMultiplierUp", deserialize_with = "from_str_to_f64")]
        bid_multiplier_up: f64,
        #[serde(rename = "bidMultiplierDown", deserialize_with = "from_str_to_f64")]
        bid_multiplier_down: f64,
        #[serde(rename = "askMultiplierUp", deserialize_with = "from_str_to_f64")]
        ask_multiplier_up: f64,
        #[serde(rename = "askMultiplierDown", deserialize_with = "from_str_to_f64")]
        ask_multiplier_down: f64,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: i32,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty", deserialize_with = "from_str_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "from_str_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "from_str_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(rename = "minNotional", deserialize_with = "from_str_to_f64")]
        min_notional: f64,
        #[serde(rename = "applyToMarket")]
        apply_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: i32,
    },
    #[serde(rename = "NOTIONAL")]
    Notional {
        #[serde(rename = "minNotional", deserialize_with = "from_str_to_f64")]
        min_notional: f64,
        #[serde(rename = "applyMinToMarket")]
        apply_min_to_market: bool,
        #[serde(rename = "maxNotional", deserialize_with = "from_str_to_f64")]
        max_notional: f64,
        #[serde(rename = "applyMaxToMarket")]
        apply_max_to_market: bool,
        #[serde(rename = "avgPriceMins")]
        avg_price_mins: i32,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    IcebergParts {
        limit: i32,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        #[serde(rename = "minQty", deserialize_with = "from_str_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "from_str_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "from_str_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_num_orders: i32,
    },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: i32,
    },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max_num_iceberg_orders: i32,
    },
    #[serde(rename = "MAX_POSITION")]
    MaxPosition {
        #[serde(rename = "maxPosition", deserialize_with = "from_str_to_f64")]
        max_position: f64,
    },
    #[serde(rename = "TRAILING_DELTA")]
    TrailingDelta {
        #[serde(rename = "minTrailingAboveDelta")]
        min_trailing_above_delta: i32,
        #[serde(rename = "maxTrailingAboveDelta")]
        max_trailing_above_delta: i32,
        #[serde(rename = "minTrailingBelowDelta")]
        min_trailing_below_delta: i32,
        #[serde(rename = "maxTrailingBelowDelta")]
        max_trailing_below_delta: i32,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "filterType")]
pub enum ExchangeFilter {
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(rename = "maxNumOrders")]
        max_num_orders: i32,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "maxNumAlgoOrders")]
        max_num_algo_orders: i32,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ICEBERG_ORDERS")]
    MaxNumIcebergOrders {
        #[serde(rename = "maxNumIcebergOrders")]
        max_num_iceberg_orders: i32,
    },
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "filterType")]
pub enum FutureSymbolFilter {
    #[serde(rename = "PRICE_FILTER")]
    PriceFilter {
        #[serde(rename = "minPrice", deserialize_with = "from_str_to_f64")]
        min_price: f64,
        #[serde(rename = "maxPrice", deserialize_with = "from_str_to_f64")]
        max_price: f64,
        #[serde(rename = "tickSize", deserialize_with = "from_str_to_f64")]
        tick_size: f64,
    },
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "minQty", deserialize_with = "from_str_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "from_str_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "from_str_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    MarketLotSize {
        #[serde(rename = "minQty", deserialize_with = "from_str_to_f64")]
        min_qty: f64,
        #[serde(rename = "maxQty", deserialize_with = "from_str_to_f64")]
        max_qty: f64,
        #[serde(rename = "stepSize", deserialize_with = "from_str_to_f64")]
        step_size: f64,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    MaxNumOrders {
        #[serde(rename = "limit")]
        limit: i32,
    },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    MaxNumAlgoOrders {
        #[serde(rename = "limit")]
        limit: i32,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    MinNotional {
        #[serde(rename = "notional", deserialize_with = "from_str_to_f64")]
        min_notional: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    PercentPrice {
        #[serde(rename = "multiplierUp", deserialize_with = "from_str_to_f64")]
        multiplier_up: f64,
        #[serde(rename = "multiplierDown", deserialize_with = "from_str_to_f64")]
        multiplier_down: f64,
        #[serde(rename = "multiplierDecimal")]
        multiplier_decimal: String,

    },
}