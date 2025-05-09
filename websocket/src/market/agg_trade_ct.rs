use crate::market::types::agg_trade::{AggTradeStream, AggTradeStreamPayload};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type AggTradeResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<AggTradeStreamPayload>>> + Send>>;

pub struct AggTradeClient {
    websocket_client: WebsocketClient<AggTradeStream>,
}

#[async_trait]
impl BinanceWebsocketAdaptor for AggTradeClient {
    type CLIENT = AggTradeClient;
    type INPUT = Symbol;
    type OUTPUT = AggTradeStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<AggTradeStream>::new_with_uri::<AggTradeStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(agg_trade_payload_process(trade_stream, process));
        AggTradeClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await;
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(AggTradeStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|item| AggTradeStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(AggTradeStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| AggTradeStream::new(symbol))
            .collect::<Vec<AggTradeStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| item.get_symbol().clone())
            .collect()
    }
}

pub(crate) async fn agg_trade_payload_process<P>(
    trade_response_stream: AggTradeResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}