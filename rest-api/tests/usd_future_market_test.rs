use client::rest::client::BinanceClient;
use client::rest::config::Config;
use client::rest::rest_client::BinanceRestClient;
use lazy_static::lazy_static;
// use rest_api::usd_future_market_ct::UsdFutureMarketClient;
//
// lazy_static! {
//     static ref CLIENT: UsdFutureMarketClient<BinanceRestClient> =
//         UsdFutureMarketClient::new(BinanceRestClient::build_client(Config::new_default()));
// }
//
// #[tokio::test]
// async fn exchange_info_test() {
//     let resp = CLIENT.get_exchange().await;
//     println!("{:?}", resp);
// }
