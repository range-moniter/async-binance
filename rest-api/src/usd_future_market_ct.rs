use crate::types::market::future_exchange::FutureExchangeInfoResp;
use crate::types::market::order_book::{CommonReq, OrderBookResp};
use crate::types::market::trade_list::{AggTradeListReq, AggTradeListResp, LookupTradeListReq, TradeListResp};
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use general::result::BinanceResult;

pub struct UsdFutureMarketClient<T> {
    client: T,
    domain: String,
}

impl<T> UsdFutureMarketClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> UsdFutureMarketClient<T> {
        UsdFutureMarketClient {
            client,
            domain: "fapi.binance.com".to_string(),
        }
    }

    /// https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Exchange-Information
    pub async fn exchange_info(&self) -> BinanceResult<FutureExchangeInfoResp> {
        self.client
            .get(
                None::<String>,
                "/fapi/v1/exchangeInfo",
                self.domain.as_str(),
                RequestExtension::none_auth_api(1),
            )
            .await
    }

    /// https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Order-Book
    pub async fn order_book(&self, request: CommonReq) -> BinanceResult<OrderBookResp> {
        let weight = if request.get_limit() <= 100 {
            2
        } else if request.get_limit() >= 101 && request.get_limit() <= 500 {
            5
        } else if request.get_limit() >= 501 && request.get_limit() <= 1000 {
            10
        } else {
            20
        };
        self.client
            .get(
                Some(request),
                "/fapi/v1/depth",
                self.domain.as_str(),
                RequestExtension::none_auth_api(weight),
            )
            .await
    }

    /// https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Recent-Trades-List
    pub async fn trade_list(&self, request: CommonReq) -> BinanceResult<Vec<TradeListResp>> {
        self.client
            .get_multiple(
                Some(request),
                "/fapi/v1/trades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(5),
            )
            .await
    }

    ///  https://developers.binance.com/docs/derivatives/usds-margined-futures/market-data/rest-api/Old-Trades-Lookup
    ///  Error Response: 2014
    pub async fn historical_trade_list(
        &self,
        request: LookupTradeListReq,
    ) -> BinanceResult<Vec<TradeListResp>> {
        self.client
            .get_multiple(
                Some(request),
                "/fapi/v1/historicalTrades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(20),
            )
            .await
    }

    pub async fn aggregate_trade_list(
        &self,
        request: AggTradeListReq,
    ) -> BinanceResult<Vec<AggTradeListResp>> {
        self.client
            .get_multiple(
                Some(request),
                "/fapi/v1/aggTrades",
                self.domain.as_str(),
                RequestExtension::none_auth_api(20),
            )
            .await
    }
}
