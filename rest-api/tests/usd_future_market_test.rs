use env_logger::Builder;
use client::rest::client::BinanceClient;
use client::rest::config::Config;
use client::rest::rest_client::BinanceRestClient;
use lazy_static::lazy_static;
use rest_api::usd_future_market_ct::UsdFutureMarketClient;

lazy_static! {
    static ref CLIENT: UsdFutureMarketClient<BinanceRestClient> =
        UsdFutureMarketClient::new(BinanceRestClient::build_client(Config::new_default()));
}

#[tokio::test]
async fn exchange_info_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let resp = CLIENT.exchange_info().await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    }else {
        print!("{:#?}", resp.unwrap_err());
    }
}