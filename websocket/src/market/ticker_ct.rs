use crate::market::types::symbol_ticker::{SymbolTickerPayload, SymbolTickerStream};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
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
#[async_trait]
impl BinanceWebsocketAdaptor for SymbolTickerClient {
    type CLIENT = SymbolTickerClient;
    type INPUT = Symbol;
    type OUTPUT = SymbolTickerPayload;

    async fn create_client<P>(process: P) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolTickerStream>::new::<SymbolTickerPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_ticker_payload_process(trade_stream, process));
        SymbolTickerClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(SymbolTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(SymbolTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolTickerStream::new(symbol))
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
    use client::stream::stream::DefaultStreamPayloadProcess;
    use env_logger::Builder;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Info)
            .init();

        let mut symbol_ticker_client = BinanceMarketWebsocketClient::symbol_ticker(DefaultStreamPayloadProcess::new()).await;

        symbol_ticker_client
            .subscribe_item(Symbol::new("ARKUSDT"))
            .await;

        sleep(Duration::from_secs(5)).await;

        symbol_ticker_client
            .subscribe_item(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(5)).await;

        symbol_ticker_client
            .subscribe_item(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(5)).await;

        println!("send close message");

        symbol_ticker_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
