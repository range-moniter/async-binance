use client::rest::client::BinanceClient;
use client::rest::config::Config;
use client::rest::rest_client::BinanceRestClient;
use env_logger::Builder;
use lazy_static::lazy_static;
use rest_api::types::market::order_book::CommonReq;
use rest_api::types::market::trade_list::{AggTradeListReq, LookupTradeListReq};
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
    } else {
        print!("{:#?}", resp.unwrap_err());
    }
}

#[tokio::test]
async fn order_book() {
    let resp = CLIENT.order_book(CommonReq::new("BTCUSDT", 20)).await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    } else {
        print!("{:#?}", resp.unwrap_err());
    }
}

#[tokio::test]
async fn recent_trade_list() {
    let resp = CLIENT.trade_list(CommonReq::new("BTCUSDT", 500)).await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    } else {
        print!("{:#?}", resp.unwrap_err());
    }
}

#[tokio::test]
async fn historical_recent_trade_list() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let resp = CLIENT
        .historical_trade_list(LookupTradeListReq::new("BTCUSDT"))
        .await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    } else {
        print!("{:#?}", resp.unwrap_err());
    }
}

#[tokio::test]
async fn aggregate_trade_list() {
    let resp = CLIENT
        .aggregate_trade_list(AggTradeListReq::new("BTCUSDT"))
        .await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    } else {
        print!("{:#?}", resp.unwrap_err());
    }
}
