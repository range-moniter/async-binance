use crate::market::types::kline::{KlineStream, KlineStreamPayload};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::interval::Interval;
use general::enums::timezone::Timezone;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type KlineResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<KlineStreamPayload>>> + Send>>;

pub struct KlineClient {
    websocket_client: WebsocketClient<KlineStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for KlineClient {
    type CLIENT = KlineClient;
    type INPUT = (Symbol, Interval, Option<Timezone>);
    type OUTPUT = KlineStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<KlineStream>::new_with_uri::<KlineStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(kline_payload_process(trade_stream, process));
        KlineClient {
            websocket_client: client,
        }
    }


    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(KlineStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, interval, timezone)| KlineStream::new(symbol, interval, timezone))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(KlineStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, interval, timezone)| KlineStream::new(symbol, interval, timezone))
            .collect::<Vec<KlineStream>>();
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
                (
                    item.get_symbol(),
                    item.get_kline_type(),
                    item.get_timezone(),
                )
            })
            .collect()
    }
}

pub(crate) async fn kline_payload_process<P>(
    trade_response_stream: KlineResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}
