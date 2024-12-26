use crate::SocketOperator;
use crate::market::types::average_price::{AveragePricePayload, AveragePriceStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;

pub type AvgPriceResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<AveragePricePayload>>> + Send>>;

pub struct AveragePriceClient {
    websocket_client: WebsocketClient<AveragePriceStream>,
}

impl AveragePriceClient {
    pub fn new(websocket_client: WebsocketClient<AveragePriceStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(AveragePriceStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|item| AveragePriceStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(AveragePriceStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| AveragePriceStream::new(symbol))
            .collect::<Vec<AveragePriceStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }
    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}

impl SocketOperator<AveragePricePayload> for AveragePriceClient {
    async fn close(self) {
        self.close().await
    }
    async fn subscribe_with_entity(&mut self, item: AveragePriceStream) {
        self.websocket_client.subscribe_single(item).await.unwrap();
    }
    async fn subscribe_with_entities(&mut self, items: Vec<AveragePriceStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap();
    }
    async fn unsubscribe_with_entity(&mut self, item: AveragePriceStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap();
    }
    async fn unsubscribe_with_entities(&mut self, items: Vec<AveragePriceStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap();
    }
    fn get_all_entities(&self) -> HashSet<AveragePriceStream> {
        self.websocket_client.get_all_subscribers()
    }
}

pub(crate) async fn avg_price_payload_process<P>(
    trade_response_stream: AvgPriceResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<AveragePricePayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use env_logger::Builder;
    use std::time::Duration;
    use tokio::time::sleep;
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
