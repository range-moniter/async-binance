# async-binance


### This is a lightweight library that works as a connector to [Binance public API](https://github.com/binance/binance-spot-api-docs)
#### Some APIs are not supported yet; contributions are welcome.


## Rest API

```rust
    use crate::types::order::general::create::CreateOrderReqBuilder;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use client::rest::config::Config;
    use client::rest::rest_client::BinanceRestClient;
    use general::enums::general::TimeInForce;
    use general::enums::order::{OrderSide, OrderType};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: OrderClient<BinanceRestClient> =
            OrderClient::new(BinanceRestClient::build_client(Config::new_default()));
        static ref CERTIFICATE: Certificate = Certificate::new("", "");
    }
    #[tokio::main]
    async fn main() {
        let req = CreateOrderReqBuilder::new_builder()
            .symbol("DOGEUSDT")
            .side(OrderSide::BUY)
            .order_type(OrderType::LIMIT)
            .time_in_force(TimeInForce::GTC)
            .quantity(BigDecimal::from_f32(10.0).unwrap().with_scale(2))
            .price(BigDecimal::from_f32(0.4).unwrap().with_scale(4))
            .build();
        match req {
            Ok(req) => {
                let resp = CLIENT.create_order(req, CERTIFICATE.clone()).await;
                println!("{:?}", resp)
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        } 
    }
```


## Websocket

```rust
    use super::*;
use crate::market_socket_ct::BinanceMarketWebsocketClient;
use client::stream::stream::DefaultStreamPayloadProcess;
use env_logger::Builder;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_trade() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let process = DefaultStreamPayloadProcess::<TradeStreamPayload>::new();

    let mut trade_client = BinanceMarketWebsocketClient::trade(process).await;

    trade_client.subscribe_item(Symbol::new("ETHUSDT")).await;

    sleep(Duration::from_secs(5)).await;

    println!("send close message");

    trade_client.close().await;

    println!("client close message");
    sleep(Duration::from_millis(1000000)).await;
}
```

## Rest API# async-binance
