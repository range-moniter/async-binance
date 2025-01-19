use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::interval::Interval;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;
use general::enums::contract_type::ContractType;
use crate::market::types::continuous_kline::{ContinuousKlineStream, ContinuousKlineStreamPayload};

pub type ContinuousKlineResponseStream =
Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<ContinuousKlineStreamPayload>>> + Send>>;

pub struct ContinuousKlineClient {
    websocket_client: WebsocketClient<ContinuousKlineStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for ContinuousKlineClient {
    type CLIENT = ContinuousKlineClient;
    type INPUT = (Symbol, ContractType, Interval);
    type OUTPUT = ContinuousKlineStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<ContinuousKlineStream>::new_with_uri::<ContinuousKlineStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(continuous_kline_payload_process(trade_stream, process));
        ContinuousKlineClient {
            websocket_client: client,
        }
    }


    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(ContinuousKlineStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, contract_type, interval)| ContinuousKlineStream::new(symbol, contract_type, interval))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(ContinuousKlineStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, contract_type, interval)| ContinuousKlineStream::new(symbol, contract_type, interval))
            .collect::<Vec<ContinuousKlineStream>>();
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
                    item.get_contract_type(),
                    item.get_interval()
                )
            })
            .collect()
    }
}

pub(crate) async fn continuous_kline_payload_process<P>(
    trade_response_stream: ContinuousKlineResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<ContinuousKlineStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

// todo