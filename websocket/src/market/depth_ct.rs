use crate::SocketOperator;
use crate::market::types::depth::{DepthStream, DepthStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;
use async_trait::async_trait;

pub type DepthResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<DepthStreamPayload>>> + Send>>;

pub struct DepthClient {
    websocket_client: WebsocketClient<DepthStream>,
}

impl DepthClient {
    pub fn new(websocket_client: WebsocketClient<DepthStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(DepthStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_multi(&mut self, params: Vec<Symbol>) {
        let params = params
            .into_iter()
            .map(|symbol| DepthStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(DepthStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| DepthStream::new(symbol))
            .collect::<Vec<DepthStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}
#[async_trait]
impl SocketOperator<DepthStream> for DepthClient {
    async fn close(self) {
        self.close().await
    }

    async fn subscribe_with_entity(&mut self, item: DepthStream) {
        self.websocket_client.subscribe_single(item).await.unwrap();
    }

    async fn subscribe_with_entities(&mut self, items: Vec<DepthStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap();
    }

    async fn unsubscribe_with_entity(&mut self, item: DepthStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap();
    }

    async fn unsubscribe_with_entities(&mut self, items: Vec<DepthStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap();
    }

    fn get_all_entities(&self) -> HashSet<DepthStream> {
        self.websocket_client.get_all_subscribers()
    }
}

pub(crate) async fn depth_payload_process<P>(
    trade_response_stream: DepthResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<DepthStreamPayload> + Send + 'static,
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

        let mut book_depth_client = BinanceMarketWebsocketClient::depth().await;

        book_depth_client.subscribe(Symbol::new("ARKUSDT")).await;

        sleep(Duration::from_secs(15)).await;

        book_depth_client.subscribe(Symbol::new("FILUSDT")).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        book_depth_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
