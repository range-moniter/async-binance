use crate::market::types::depth::DepthStreamPayload;
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::speed::Speed;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;
use general::enums::level::Level;
use crate::market::types::book_depth::BookDepthStream;

pub type PartialDepthResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<DepthStreamPayload>>> + Send>>;

pub struct PartialDepthClient {
    websocket_client: WebsocketClient<BookDepthStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for PartialDepthClient {
    type CLIENT = PartialDepthClient;
    type INPUT = (Symbol, Level, Option<Speed>);
    type OUTPUT = DepthStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<BookDepthStream>::new_with_uri::<DepthStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(partial_depth_payload_process(trade_stream, process));
        PartialDepthClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(BookDepthStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol,level, speed)| BookDepthStream::new(symbol, level, speed))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(BookDepthStream::new(input.0, input.1, input.2))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|(symbol, level, speed)| BookDepthStream::new(symbol, level,  speed))
            .collect::<Vec<BookDepthStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| (item.get_symbol(), item.get_level(),  item.get_speed()))
            .collect()
    }
}

pub(crate) async fn partial_depth_payload_process<P>(
    trade_response_stream: PartialDepthResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<DepthStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}