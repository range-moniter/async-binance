use crate::types::market::exchange::{ExchangeReq, ExchangeResp};
use crate::types::market::kline::{KlineReq, KlineResp, KlineTupleResp};
use crate::types::market::order_book::{CommonReq, OrderBookResp};
use crate::types::market::price::{
    AvgPriceReq, AvgPriceResp, PriceChangeTickerReq, PriceChangeTickerStatResp,
    PriceTradeDayTickerReq, PriceTradeDayTickerResp, RollingWindowPriceChangeStatReq,
    RollingWindowPriceChangeStatResp, SymbolOrderBookResp, SymbolPriceTickerResp, SymbolReq,
};
use crate::types::market::trade_list::{
    AggTradeListReq, AggTradeListResp, LookupTradeListReq, TradeListResp,
};
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use general::result::BinanceResult;

pub struct SpotMarketClient<T> {
    client: T,
    domain: String,
}

impl<T> SpotMarketClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> SpotMarketClient<T> {
        SpotMarketClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }

    pub async fn get_exchange(&self, request: ExchangeReq) -> BinanceResult<ExchangeResp> {
        self.client
            .get(
                Some(request),
                "/api/v3/exchangeInfo",
                self.domain.as_str(),
                RequestExtension::none_auth_api(20),
            )
            .await
    }

    pub async fn get_order_book(&self, request: CommonReq) -> BinanceResult<OrderBookResp> {
        let weight = if request.get_limit() <= 100 {
            5
        } else if request.get_limit() >= 101 && request.get_limit() <= 500 {
            25
        } else if request.get_limit() >= 501 && request.get_limit() <= 1000 {
            50
        } else {
            250
        };
        self.client
            .get(
                Some(request),
                "/api/v3/depth",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight),
            )
            .await
    }

    pub async fn get_recent_trade_lists(
        &self,
        request: CommonReq,
    ) -> BinanceResult<Vec<TradeListResp>> {
        self.client
            .get_multiple(
                Some(request),
                "/api/v3/trades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(25),
            )
            .await
    }

    pub async fn get_old_trade_lists(
        &self,
        request: LookupTradeListReq,
    ) -> BinanceResult<Vec<TradeListResp>> {
        self.client
            .get_multiple(
                Some(request),
                "/api/v3/historicalTrades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(25),
            )
            .await
    }

    pub async fn get_agg_trade_list(
        &self,
        request: AggTradeListReq,
    ) -> BinanceResult<Vec<AggTradeListResp>> {
        self.client
            .get(
                Some(request),
                "/api/v3/aggTrades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await
    }

    pub async fn get_kline(&self, request: KlineReq) -> BinanceResult<Vec<KlineResp>> {
        let items: BinanceResult<Vec<KlineTupleResp>> = self
            .client
            .get_multiple(
                Some(request),
                "/api/v3/klines",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await;
        items.map(|items| items.into_iter().map(Into::into).collect())
    }

    pub async fn get_ui_kline(&self, request: KlineReq) -> BinanceResult<Vec<KlineResp>> {
        let items: BinanceResult<Vec<KlineTupleResp>> = self
            .client
            .get(
                Some(request),
                "/api/v3/uiKlines",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await;
        items.map(|items| items.into_iter().map(Into::into).collect())
    }

    pub async fn get_avg_price(&self, request: AvgPriceReq) -> BinanceResult<AvgPriceResp> {
        self.client
            .get(
                Some(request),
                "/api/v3/avgPrice",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await
    }

    pub async fn get_price_ticker_24hr(
        &self,
        request: PriceChangeTickerReq,
    ) -> BinanceResult<PriceChangeTickerStatResp> {
        if !request.symbols_is_none() || request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/24hr",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await
    }

    pub async fn get_price_tickers_24hr(
        &self,
        request: PriceChangeTickerReq,
    ) -> BinanceResult<Vec<PriceChangeTickerStatResp>> {
        if request.symbols_is_none() || !request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        let len = request.symbols_len();
        let weight = if len <= 20 {
            2
        } else if len >= 21 && len <= 100 {
            40
        } else {
            80
        };
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/24hr",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight),
            )
            .await
    }

    pub async fn get_trading_day_ticker(
        &self,
        request: PriceTradeDayTickerReq,
    ) -> BinanceResult<PriceTradeDayTickerResp> {
        if !request.symbols_is_none() || request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/tradingDay",
                self.domain.as_str(),
                RequestExtension::none_auth_api(4),
            )
            .await
    }

    pub async fn get_trading_day_tickers(
        &self,
        request: PriceTradeDayTickerReq,
    ) -> BinanceResult<Vec<PriceTradeDayTickerResp>> {
        if request.symbols_is_none() || !request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        let weight = if request.get_symbols_len() > 50 {
            200
        } else {
            request.get_symbols_len() * 4
        };
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/tradingDay",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight as u32),
            )
            .await
    }

    pub async fn get_symbol_price_ticker(
        &self,
        request: SymbolReq,
    ) -> BinanceResult<SymbolPriceTickerResp> {
        if !request.symbols_is_none() || request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/price",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await
    }

    pub async fn get_symbol_price_tickers(
        &self,
        request: SymbolReq,
    ) -> BinanceResult<Vec<SymbolPriceTickerResp>> {
        if request.symbols_is_none() || !request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/price",
                self.domain.as_str(),
                RequestExtension::none_auth_api(4),
            )
            .await
    }

    pub async fn get_symbol_order_book_ticker(
        &self,
        request: SymbolReq,
    ) -> BinanceResult<SymbolOrderBookResp> {
        if !request.symbols_is_none() || request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/bookTicker",
                self.domain.as_str(),
                RequestExtension::none_auth_api(2),
            )
            .await
    }

    pub async fn get_symbol_order_book_tickers(
        &self,
        request: SymbolReq,
    ) -> BinanceResult<Vec<SymbolOrderBookResp>> {
        if request.symbols_is_none() || !request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker/bookTicker",
                self.domain.as_str(),
                RequestExtension::none_auth_api(4),
            )
            .await
    }

    pub async fn get_rolling_window_price_change_stat(
        &self,
        request: RollingWindowPriceChangeStatReq,
    ) -> BinanceResult<RollingWindowPriceChangeStatResp> {
        if !request.symbols_is_none() || request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        self.client
            .get(
                Some(request),
                "/api/v3/ticker",
                self.domain.as_str(),
                RequestExtension::none_auth_api(4),
            )
            .await
    }

    pub async fn get_rolling_window_price_change_stats(
        &self,
        request: RollingWindowPriceChangeStatReq,
    ) -> BinanceResult<Vec<RollingWindowPriceChangeStatResp>> {
        if request.symbols_is_none() || !request.symbol_is_none() {
            panic!("params error: params symbols must be none, and param symbol need a value")
        }
        let weight = if request.get_symbols_len() > 50 {
            200
        } else {
            request.get_symbols_len() * 4
        };
        self.client
            .get(
                Some(request),
                "/api/v3/ticker",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight as u32),
            )
            .await
    }
}