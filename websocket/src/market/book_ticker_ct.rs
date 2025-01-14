use crate::market::types::symbol_book_ticker::{SymbolBookTickerPayload, SymbolBookTickerStream};
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type SymbolBookTickerResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolBookTickerPayload>>> + Send>>;

pub struct SymbolBookTickerClient {
    websocket_client: WebsocketClient<SymbolBookTickerStream>,
}

impl<P> BinanceWebsocketAdaptor<P> for SymbolBookTickerClient
where
    P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
{
    type CLIENT = SymbolBookTickerClient;
    type INPUT = Symbol;
    type OUTPUT = SymbolBookTickerPayload;

    async fn create_client(process: P) -> Self::CLIENT {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolBookTickerStream>::new::<SymbolBookTickerPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_book_ticker_payload_process(trade_stream, process));
        SymbolBookTickerClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(SymbolBookTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolBookTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(SymbolBookTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolBookTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| item.get_symbol())
            .collect()
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
