use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use std::pin::Pin;
use crate::market::types::liquidation_order::{LiquidationOrderStreamPayload, TotalLiquidationOrderStream};

pub type TotalLiquidationOrderResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<LiquidationOrderStreamPayload>>> + Send>>;

pub struct TotalLiquidationOrderClient {
    websocket_client: WebsocketClient<TotalLiquidationOrderStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for TotalLiquidationOrderClient {
    type CLIENT = TotalLiquidationOrderClient;
    type INPUT = TotalLiquidationOrderStream;
    type OUTPUT = LiquidationOrderStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<TotalLiquidationOrderStream>::new_with_uri::<LiquidationOrderStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(total_liquidation_order_payload_process(trade_stream, process));
        TotalLiquidationOrderClient {
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
            .map(|item| {
               item.clone()
            })
            .collect()
    }
}

pub(crate) async fn total_liquidation_order_payload_process<P>(
    trade_response_stream: TotalLiquidationOrderResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<LiquidationOrderStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

// todo test