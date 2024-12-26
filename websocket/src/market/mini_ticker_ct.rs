use crate::SocketOperator;
use crate::market::types::symbol_mini_ticker::{SymbolMiniTickerPayload, SymbolMiniTickerStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;
use async_trait::async_trait;

pub type MiniTickerResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolMiniTickerPayload>>> + Send>>;

pub struct SymbolMiniTickerClient {
    websocket_client: WebsocketClient<SymbolMiniTickerStream>,
}

impl SymbolMiniTickerClient {
    pub fn new(websocket_client: WebsocketClient<SymbolMiniTickerStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_mini_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(SymbolMiniTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_mini_ticker_multi(&mut self, params: Vec<Symbol>) {
        let params = params
            .into_iter()
            .map(|symbol| SymbolMiniTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe_mini_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(SymbolMiniTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_mini_ticker_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| SymbolMiniTickerStream::new(symbol))
            .collect::<Vec<_>>();
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
impl SocketOperator<SymbolMiniTickerStream> for SymbolMiniTickerClient {
    async fn close(self) {
        self.close().await
    }

    async fn subscribe_with_entity(&mut self, item: SymbolMiniTickerStream) {
        self.websocket_client.subscribe_single(item).await.unwrap()
    }

    async fn subscribe_with_entities(&mut self, items: Vec<SymbolMiniTickerStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entity(&mut self, item: SymbolMiniTickerStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entities(&mut self, items: Vec<SymbolMiniTickerStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap()
    }

    fn get_all_entities(&self) -> HashSet<SymbolMiniTickerStream> {
        self.websocket_client.get_all_subscribers()
    }
}
pub(crate) async fn symbol_mini_ticker_payload_process<P>(
    trade_response_stream: MiniTickerResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use env_logger::Builder;
    use log::Level;
    use std::time::Duration;
    use tokio::time::sleep;
    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut symbol_mini_ticker_client =
            BinanceMarketWebsocketClient::symbol_mini_ticker().await;

        symbol_mini_ticker_client
            .subscribe_mini_ticker(Symbol::new("ARKUSDT"))
            .await;

        sleep(Duration::from_secs(15)).await;

        symbol_mini_ticker_client
            .subscribe_mini_ticker(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        symbol_mini_ticker_client
            .unsubscribe_mini_ticker(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        symbol_mini_ticker_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
