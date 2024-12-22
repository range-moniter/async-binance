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

pub struct MarketClient<T> {
    client: T,
    domain: String,
}

impl<T> MarketClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> MarketClient<T> {
        MarketClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }

    pub async fn get_exchange(&self, request: ExchangeReq) -> BinanceResult<ExchangeResp> {
        self.client
            .get(
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
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
                request,
                "/api/v3/ticker",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight as u32),
            )
            .await
    }
}
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use client::rest::config::Config;
    use client::rest::rest_client::BinanceRestClient;
    use super::*;
    use crate::types::market::kline::KlineReq;
    use general::enums::interval::Interval;


    lazy_static!{
        static ref CLIENT: MarketClient<BinanceRestClient> = MarketClient::new(BinanceRestClient::build_client(Config::new_default()));
    }


    #[tokio::test]
    async fn test_get_exchange() {
        let mut req = ExchangeReq::new();
        req.set_symbol("BTCUSDT");
        let resp = CLIENT.get_exchange(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_order_book() {
        let req = CommonReq::new("BTCUSDT", 1);
        let resp = CLIENT.get_order_book(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_recent_trade_list() {
        let req = CommonReq::new("BTCUSDT", 500);
        let resp = CLIENT.get_recent_trade_lists(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_old_trade_list() {
        let req = LookupTradeListReq::new("BTCUSDT");
        let resp = CLIENT.get_old_trade_lists(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_agg_trade_list() {
        let req = AggTradeListReq::new("BTCUSDT");
        let resp = CLIENT.get_agg_trade_list(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_kline_candlestick() {
        let req = KlineReq::new("BTCUSDT", Interval::Minute3);
        let resp = CLIENT.get_kline(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_ui_kline() {
        let req = KlineReq::new("BTCUSDT", Interval::Minute3);
        let resp = CLIENT.get_ui_kline(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_avg_price() {
        let req = AvgPriceReq::new("BTCUSDT");
        let resp = CLIENT.get_avg_price(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_price_ticker() {
        let req = PriceChangeTickerReq::new_with_single("BTCUSDT");
        let resp = CLIENT.get_price_ticker_24hr(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_price_tickers() {
        let req = PriceChangeTickerReq::new_with_multiple(vec!["BTCUSDT", "ETHUSDT"]);
        let resp = CLIENT.get_price_tickers_24hr(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_price_trade_day_ticker() {
        let req = PriceTradeDayTickerReq::new_with_single("BTCUSDT");
        let resp = CLIENT.get_trading_day_ticker(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_price_trade_day_tickers() {
        let req = PriceTradeDayTickerReq::new_with_multiple(vec!["BTCUSDT", "ETHUSDT"]);
        let resp = CLIENT.get_trading_day_tickers(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_symbol_price_ticker() {
        let req = SymbolReq::new_with_single("BTCUSDT");
        let resp = CLIENT.get_symbol_price_ticker(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_symbol_price_tickers() {
        let req = SymbolReq::new_with_multiple(vec!["BTCUSDT", "ETHUSDT"]);
        let resp = CLIENT.get_symbol_price_tickers(req).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_symbol_order_book_ticker() {
        let req = SymbolReq::new_with_single("BTCUSDT");
        let resp = CLIENT.get_symbol_order_book_ticker(req).await;
        println!("{:?}", resp);
    }
    #[tokio::test]
    async fn test_get_symbol_order_book_tickers() {
        let req = SymbolReq::new_with_multiple(vec!["BTCUSDT", "ETHUSDT"]);
        let resp = CLIENT.get_symbol_order_book_tickers(req).await;
        println!("{:?}", resp);
    }
    #[tokio::test]
    async fn test_get_rolling_window_price_change_static() {
        let req = RollingWindowPriceChangeStatReq::new_with_single("BTCUSDT");
        let resp = CLIENT.get_rolling_window_price_change_stat(req).await;
        println!("{:?}", resp);
    }
    #[tokio::test]
    async fn test_get_rolling_window_price_change_statics() {
        let req = RollingWindowPriceChangeStatReq::new_with_multiple(vec!["BTCUSDT", "ETHUSDT"]);
        let resp = CLIENT.get_rolling_window_price_change_stats(req).await;
        println!("{:?}", resp);
    }
}
