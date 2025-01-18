use crate::market::types::book_depth::{BookDepthStream, BookDepthStreamPayload};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::enums::level::Level;
use general::enums::speed::Speed;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type BookDepthResponseStream =
Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<BookDepthStreamPayload>>> + Send>>;

pub struct BookDepthClient {
    websocket_client: WebsocketClient<BookDepthStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for BookDepthClient
{
    type CLIENT = BookDepthClient;
    type INPUT = (Symbol, Level, Option<Speed>);
    type OUTPUT = BookDepthStreamPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<BookDepthStream>::new_with_uri::<BookDepthStreamPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(book_depth_payload_process(trade_stream, process));
        BookDepthClient {
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
            .map(|(symbol, level, speed)| BookDepthStream::new(symbol, level, speed))
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
            .map(|(symbol, level, speed)| BookDepthStream::new(symbol, level, speed))
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
            .map(|item| (item.get_symbol(), item.get_level(), item.get_speed()))
            .collect()
    }
}
pub(crate) async fn book_depth_payload_process<P>(
    trade_response_stream: BookDepthResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<BookDepthStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger::Builder;
    use std::time::Duration;
    use tokio::time::sleep;
    use client::stream::stream::DefaultStreamPayloadProcess;
    use crate::spot_market_socket_ct::BinanceSpotMarketWebsocketClient;

    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut book_depth_client = BinanceSpotMarketWebsocketClient::book_depth(DefaultStreamPayloadProcess::new()).await;

        book_depth_client
            .subscribe_item((Symbol::new("ARKUSDT"), Level::L1, None))
            .await;

        sleep(Duration::from_secs(15)).await;

        book_depth_client
            .subscribe_item((Symbol::new("FILUSDT"), Level::L1, None))
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        book_depth_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
