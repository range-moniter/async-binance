use crate::SocketOperator;
use crate::market::types::symbol_book_ticker::{SymbolBookTickerPayload, SymbolBookTickerStream};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;

pub type SymbolBookTickerResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolBookTickerPayload>>> + Send>>;

pub struct SymbolBookTickerClient {
    websocket_client: WebsocketClient<SymbolBookTickerStream>,
}

impl SymbolBookTickerClient {
    pub fn new(websocket_client: WebsocketClient<SymbolBookTickerStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_symbol_book_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(SymbolBookTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_symbol_book_ticker_multi(&mut self, params: Vec<Symbol>) {
        let params = params
            .into_iter()
            .map(|symbol| SymbolBookTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe_symbol_book_ticker(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(SymbolBookTickerStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_symbol_book_ticker_multi(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| SymbolBookTickerStream::new(symbol))
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

impl SocketOperator<SymbolBookTickerStream> for SymbolBookTickerClient {
    async fn close(self) {
        self.close().await
    }

    async fn subscribe_with_entity(&mut self, item: SymbolBookTickerStream) {
        self.websocket_client.subscribe_single(item).await.unwrap()
    }

    async fn subscribe_with_entities(&mut self, items: Vec<SymbolBookTickerStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entity(&mut self, item: SymbolBookTickerStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entities(&mut self, items: Vec<SymbolBookTickerStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap()
    }

    fn get_all_entities(&self) -> HashSet<SymbolBookTickerStream> {
        self.websocket_client.get_all_subscribers()
    }
}
pub(crate) async fn symbol_book_ticker_payload_process<P>(
    trade_response_stream: SymbolBookTickerResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use env_logger::Builder;
    use general::enums::interval::Interval;
    use std::time::Duration;
    use tokio::time::sleep;
    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut symbol_book_ticker_client = BinanceMarketWebsocketClient::kline().await;

        symbol_book_ticker_client
            .subscribe_kline(Symbol::new("ARKUSDT"), Interval::Second1)
            .await;

        sleep(Duration::from_secs(15)).await;

        symbol_book_ticker_client
            .subscribe_kline(Symbol::new("FILUSDT"), Interval::Second1)
            .await;

        sleep(Duration::from_secs(20)).await;

        symbol_book_ticker_client
            .unsubscribe_kline(Symbol::new("FILUSDT"), Interval::Second1)
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        symbol_book_ticker_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
