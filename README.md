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
use std::time::Duration;
use tokio::time::sleep;
use env_logger::Builder;
use crate::market_socket_ct::BinanceMarketWebsocketClient;


#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    
    
    // build book_depth client,
    let mut book_depth_client = BinanceMarketWebsocketClient::book_depth().await;
    
    // subscribe ARKUSDT socket 
    book_depth_client.subscribe(Symbol::new("ARKUSDT"),Level::L1).await;

    sleep(Duration::from_secs(15)).await;
    book_depth_client.subscribe(Symbol::new("FILUSDT"), Level::L1).await;

    sleep(Duration::from_secs(20)).await;

    println!("send close message");

    book_depth_client.close().await;

    sleep(Duration::from_secs(200)).await;
}
```

## Rest API# async-binance
