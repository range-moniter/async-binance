use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use std::pin::Pin;
use crate::market::types::symbol_ticker::{TotalSymbolTickerPayload, TotalSymbolTickerStream};

pub type TotalSymbolTickerResponseStream =
Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<TotalSymbolTickerPayload>>> + Send>>;

pub struct AllSymbolTickerClient {
    websocket_client: WebsocketClient<TotalSymbolTickerStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for AllSymbolTickerClient {
    type CLIENT = AllSymbolTickerClient;
    type INPUT = TotalSymbolTickerStream;
    type OUTPUT = TotalSymbolTickerPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<TotalSymbolTickerStream>::new_with_uri::<TotalSymbolTickerPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(all_symbol_ticker_payload_process(trade_stream, process));
        AllSymbolTickerClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(input)
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        self.websocket_client
            .subscribe_multiple(input)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(input)
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        self.websocket_client
            .unsubscribe_multiple(input)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| item.clone())
            .collect()
    }
}
pub(crate) async fn all_symbol_ticker_payload_process<P>(
    trade_response_stream: TotalSymbolTickerResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<TotalSymbolTickerPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}
