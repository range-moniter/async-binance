use lazy_static::lazy_static;
use client::rest::client::BinanceClient;
use client::rest::config::Config;
use client::rest::rest_client::BinanceRestClient;
use rest_api::spot_market_ct::SpotMarketClient;
use rest_api::types::market::exchange::ExchangeReq;
use rest_api::types::market::order_book::CommonReq;

lazy_static! {
    static ref CLIENT: SpotMarketClient<BinanceRestClient> =
        SpotMarketClient::new(BinanceRestClient::build_client(Config::new_default()));
}


#[tokio::test]
async fn spot_market_exchange_info() {
    let mut req = ExchangeReq::new();
    req.set_symbol("BNBUSDT");
    let resp = CLIENT.get_exchange(req).await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    }else {
        print!("{:#?}", resp.unwrap_err());
    }
}

#[tokio::test]
async fn spot_market_get_order_book() {
    let req = CommonReq::new("ETHUSDT", 100);
    let resp = CLIENT.get_order_book(req).await;
    if let Ok(resp) = resp {
        println!("{:#?}", resp);
    }else {
        print!("{:#?}", resp.unwrap_err());
    }
}