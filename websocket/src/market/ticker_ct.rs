use crate::market::types::symbol_ticker::{SymbolTickerPayload, SymbolTickerStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type SymbolTickerResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolTickerPayload>>> + Send>>;

pub struct SymbolTickerClient {
    websocket_client: WebsocketClient<SymbolTickerStream>,
}

impl SymbolTickerClient {
    pub fn new(websocket_client: WebsocketClient<SymbolTickerStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(SymbolTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_ticker_multi(&mut self, params: Vec<Symbol>) {
        let params = params
            .into_iter()
            .map(|symbol| SymbolTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(SymbolTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_ticker_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| SymbolTickerStream::new(symbol))
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
pub(crate) async fn symbol_ticker_payload_process<P>(
    trade_response_stream: SymbolTickerResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<SymbolTickerPayload> + Send + 'static,
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

        let mut symbol_ticker_client = BinanceMarketWebsocketClient::symbol_ticker().await;

        symbol_ticker_client
            .subscribe_ticker(Symbol::new("ARKUSDT"))
            .await;

        sleep(Duration::from_secs(15)).await;

        symbol_ticker_client
            .subscribe_ticker(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        symbol_ticker_client
            .unsubscribe_ticker(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        symbol_ticker_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
