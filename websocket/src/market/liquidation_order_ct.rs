use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;
use crate::market::types::liquidation_order::{LiquidationOrderStream, LiquidationOrderStreamPayload};

pub type LiquidationOrderResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<LiquidationOrderStreamPayload>>> + Send>>;

pub struct LiquidationOrderClient {
    websocket_client: WebsocketClient<LiquidationOrderStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for LiquidationOrderClient {
    type CLIENT = LiquidationOrderClient;
    type INPUT = Symbol;
    type OUTPUT = LiquidationOrderStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<LiquidationOrderStream>::new_with_uri::<LiquidationOrderStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(liquidation_order_payload_process(trade_stream, process));
        LiquidationOrderClient {
            websocket_client: client,
        }
    }


    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(LiquidationOrderStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| LiquidationOrderStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(LiquidationOrderStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| LiquidationOrderStream::new(symbol))
            .collect::<Vec<LiquidationOrderStream>>();
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

pub(crate) async fn liquidation_order_payload_process<P>(
    trade_response_stream: LiquidationOrderResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<LiquidationOrderStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}
// todo test