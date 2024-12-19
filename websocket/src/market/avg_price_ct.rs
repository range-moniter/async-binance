use std::pin::Pin;
use futures_util::Stream;
use crate::market::types::average_price::{AveragePricePayload, AveragePriceStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use general::result::BinanceResult;
use general::symbol::Symbol;

pub type AvgPriceResponseStream = Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<AveragePricePayload>>> + Send>>;

pub struct AveragePriceClient {
    websocket_client: WebsocketClient<AveragePriceStream>,
}


impl AveragePriceClient {
    pub fn new(websocket_client: WebsocketClient<AveragePriceStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe(&mut self, symbol: Symbol) {
        self.websocket_client.subscribe_single(AveragePriceStream::new(symbol)).await.unwrap();
    }


    pub async fn subscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols.into_iter()
            .map(|item| AveragePriceStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client.subscribe_multiple(params).await.unwrap()
    }

    pub async fn unsubscribe(&mut self, symbol: Symbol) {
        self.websocket_client.unsubscribe_single(AveragePriceStream::new(symbol)).await.unwrap();
    }

    pub async fn unsubscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols.into_iter()
            .map(|symbol| AveragePriceStream::new(symbol))
            .collect::<Vec<AveragePriceStream>>();
        self.websocket_client.unsubscribe_multiple(params).await.unwrap();
    }

    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}

pub(crate) async fn avg_price_payload_process<P>(trade_response_stream: AvgPriceResponseStream,
                                                 mut processor: P)
where
    P: SocketPayloadProcess<AveragePricePayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use env_logger::Builder;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use super::*;
    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut trade_client = BinanceMarketWebsocketClient::average_price().await;

        trade_client.subscribe(Symbol::new("ARKUSDT")).await;

        sleep(Duration::from_secs(15)).await;

        trade_client.subscribe(Symbol::new("FILUSDT")).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        trade_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}