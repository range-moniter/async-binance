use crate::SocketOperator;
use crate::market::types::symbol_rolling::{SymbolRollingPayload, SymbolRollingWindowStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::window_size::WindowSize;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;

pub type SymbolRollingResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolRollingPayload>>> + Send>>;

pub struct SymbolRollingClient {
    websocket_client: WebsocketClient<SymbolRollingWindowStream>,
}

impl SymbolRollingClient {
    pub fn new(websocket_client: WebsocketClient<SymbolRollingWindowStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_symbol_rolling(&mut self, symbol: Symbol, window_size: WindowSize) {
        self.websocket_client
            .subscribe_single(SymbolRollingWindowStream::new(symbol, window_size))
            .await
            .unwrap();
    }

    pub async fn subscribe_symbol_rolling_multi(&mut self, params: Vec<(Symbol, WindowSize)>) {
        let params = params
            .into_iter()
            .map(|(symbol, window_size)| SymbolRollingWindowStream::new(symbol, window_size))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe_symbol_rolling(&mut self, symbol: Symbol, window_size: WindowSize) {
        self.websocket_client
            .unsubscribe_single(SymbolRollingWindowStream::new(symbol, window_size))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_symbol_rolling_multi(&mut self, symbols: Vec<(Symbol, WindowSize)>) {
        let params = symbols
            .into_iter()
            .map(|(symbol, interval)| SymbolRollingWindowStream::new(symbol, interval))
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
impl SocketOperator<SymbolRollingWindowStream> for SymbolRollingClient {
    async fn close(self) {
        self.close().await
    }

    async fn subscribe_with_entity(&mut self, item: SymbolRollingWindowStream) {
        self.websocket_client.subscribe_single(item).await.unwrap();
    }

    async fn subscribe_with_entities(&mut self, items: Vec<SymbolRollingWindowStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entity(&mut self, item: SymbolRollingWindowStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap();
    }

    async fn unsubscribe_with_entities(&mut self, items: Vec<SymbolRollingWindowStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap()
    }

    fn get_all_entities(&self) -> HashSet<SymbolRollingWindowStream> {
        self.websocket_client.get_all_subscribers()
    }
}
pub(crate) async fn symbol_rolling_payload_process<P>(
    trade_response_stream: SymbolRollingResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<SymbolRollingPayload> + Send + 'static,
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

        let mut symbol_rolling_client = BinanceMarketWebsocketClient::symbol_rolling_ticker().await;

        symbol_rolling_client
            .subscribe_symbol_rolling(Symbol::new("ARKUSDT"), WindowSize::FourHours)
            .await;

        sleep(Duration::from_secs(15)).await;

        symbol_rolling_client
            .subscribe_symbol_rolling(Symbol::new("FILUSDT"), WindowSize::FourHours)
            .await;

        sleep(Duration::from_secs(20)).await;

        symbol_rolling_client
            .subscribe_symbol_rolling(Symbol::new("FILUSDT"), WindowSize::FourHours)
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        symbol_rolling_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
