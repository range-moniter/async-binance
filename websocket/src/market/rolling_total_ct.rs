use crate::market::types::symbol_rolling::{TotalSymbolRollingPayload, TotalSymbolRollingStream};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::window_size::WindowSize;
use general::result::BinanceResult;
use std::pin::Pin;

pub type TotalSymbolRollingResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<TotalSymbolRollingPayload>>> + Send>>;

pub struct TotalSymbolRollingClient {
    websocket_client: WebsocketClient<TotalSymbolRollingStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for TotalSymbolRollingClient {
    type CLIENT = TotalSymbolRollingClient;
    type INPUT = WindowSize;
    type OUTPUT = TotalSymbolRollingPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<TotalSymbolRollingStream>::new_with_uri::<TotalSymbolRollingPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(total_symbol_rolling_payload_process(trade_stream, process));
        TotalSymbolRollingClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(TotalSymbolRollingStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|window_size| TotalSymbolRollingStream::new(window_size))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(TotalSymbolRollingStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|window_size| TotalSymbolRollingStream::new(window_size))
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
            .map(|item| item.get_window_size())
            .collect()
    }
}

pub(crate) async fn total_symbol_rolling_payload_process<P>(
    trade_response_stream: TotalSymbolRollingResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<TotalSymbolRollingPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}