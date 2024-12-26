use crate::SocketOperator;
use crate::market::types::agg_trade::{AggTradeStream, AggTradeStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;
use async_trait::async_trait;

pub type AggTradeResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<AggTradeStreamPayload>>> + Send>>;

pub struct AggTradeClient {
    websocket_client: WebsocketClient<AggTradeStream>,
}

impl AggTradeClient {
    pub fn new(websocket_client: WebsocketClient<AggTradeStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(AggTradeStream::new(symbol))
            .await
            .unwrap();
    }
    pub async fn subscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|item| AggTradeStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }
    pub async fn unsubscribe(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(AggTradeStream::new(symbol))
            .await
            .unwrap();
    }
    pub async fn unsubscribe_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| AggTradeStream::new(symbol))
            .collect::<Vec<AggTradeStream>>();
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
impl SocketOperator<AggTradeStream> for AggTradeClient {
    async fn close(self) {
        self.close().await;
    }
    async fn subscribe_with_entity(&mut self, item: AggTradeStream) {
        self.websocket_client.subscribe_single(item).await.unwrap();
    }
    async fn subscribe_with_entities(&mut self, items: Vec<AggTradeStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap();
    }
    async fn unsubscribe_with_entity(&mut self, item: AggTradeStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap();
    }
    async fn unsubscribe_with_entities(&mut self, items: Vec<AggTradeStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap();
    }
    fn get_all_entities(&self) -> HashSet<AggTradeStream> {
        self.websocket_client.get_all_subscribers()
    }
}

pub(crate) async fn agg_trade_payload_process<P>(
    trade_response_stream: AggTradeResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
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
    async fn test_agg_trade() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut trade_client = BinanceMarketWebsocketClient::agg_trade().await;

        trade_client.subscribe(Symbol::new("ARKUSDT")).await;

        sleep(Duration::from_secs(15)).await;

        trade_client.subscribe(Symbol::new("FILUSDT")).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        trade_client.close().await;

        sleep(Duration::from_millis(1000000)).await;
    }
}
