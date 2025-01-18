use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;
use crate::market::types::mark_price::{MarkPriceStream, MarkPriceStreamPayload};

pub type MarkPriceResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<MarkPriceStreamPayload>>> + Send>>;

pub struct MarkPriceClient {
    websocket_client: WebsocketClient<MarkPriceStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for MarkPriceClient
{
    type CLIENT = MarkPriceClient;
    type INPUT = (Symbol, bool);
    type OUTPUT = MarkPriceStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<MarkPriceStream>::new_with_uri::<MarkPriceStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(mark_price_payload_process(trade_stream, process));
        MarkPriceClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(MarkPriceStream::new(input.0, input.1))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|item| MarkPriceStream::new(item.0, item.1))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(MarkPriceStream::new(input.0, input.1))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|input| MarkPriceStream::new(input.0, input.1))
            .collect::<Vec<MarkPriceStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| (item.get_symbol(), item.is_second()))
            .collect()
    }
}

pub(crate) async fn mark_price_payload_process<P>(
    trade_response_stream: MarkPriceResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<MarkPriceStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}