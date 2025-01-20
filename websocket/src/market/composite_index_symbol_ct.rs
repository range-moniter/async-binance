use std::pin::Pin;
use async_trait::async_trait;
use futures_util::Stream;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use general::result::BinanceResult;
use general::symbol::Symbol;
use crate::market::types::composite_index_symbol::{CompositionIndexSymbolStream, CompositionIndexSymbolStreamPayload};

pub type CompositeIndexSymbolResponseStream =
Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<CompositionIndexSymbolStreamPayload>>> + Send>>;

pub struct CompositeIndexSymbolClient {
    websocket_client: WebsocketClient<CompositionIndexSymbolStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for CompositeIndexSymbolClient {
    type CLIENT = CompositeIndexSymbolClient;
    type INPUT = Symbol;
    type OUTPUT = CompositionIndexSymbolStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static
    {
        let (client, payload_receiver) =
            WebsocketClient::<CompositionIndexSymbolStream>::new_with_uri::<CompositionIndexSymbolStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(composite_index_symbol_payload_process(trade_stream, process));
        CompositeIndexSymbolClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(CompositionIndexSymbolStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| CompositionIndexSymbolStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(CompositionIndexSymbolStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| CompositionIndexSymbolStream::new(symbol))
            .collect::<Vec<CompositionIndexSymbolStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| {
                item.get_symbol()
            })
            .collect()
    }
}


pub(crate) async fn composite_index_symbol_payload_process<P>(
    trade_response_stream: CompositeIndexSymbolResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<CompositionIndexSymbolStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}
