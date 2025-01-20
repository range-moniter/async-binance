use crate::market::types::symbol_rolling::{SymbolRollingPayload, SymbolRollingWindowStream};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::window_size::WindowSize;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type SymbolRollingResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolRollingPayload>>> + Send>>;

pub struct SymbolRollingClient {
    websocket_client: WebsocketClient<SymbolRollingWindowStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for SymbolRollingClient {
    type CLIENT = SymbolRollingClient;
    type INPUT = (Symbol, WindowSize);
    type OUTPUT = SymbolRollingPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolRollingWindowStream>::new_with_uri::<SymbolRollingPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_rolling_payload_process(trade_stream, process));
        SymbolRollingClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(SymbolRollingWindowStream::new(input.0, input.1))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, window_size)| SymbolRollingWindowStream::new(symbol, window_size))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(SymbolRollingWindowStream::new(input.0, input.1))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, interval)| SymbolRollingWindowStream::new(symbol, interval))
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
            .map(|item| (item.get_symbol(), item.get_window_size()))
            .collect()
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