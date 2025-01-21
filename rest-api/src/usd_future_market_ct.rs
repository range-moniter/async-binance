use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use general::result::BinanceResult;
use crate::types::market::future_exchange::FutureExchangeInfoResp;

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
}
