use crate::market::types::trade::{TradeStream, TradeStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;
use client::stream::adaptor::BinanceWebsocketAdaptor;

pub type TradeResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<TradeStreamPayload>>> + Send>>;

pub struct TradeClient {
    websocket_client: WebsocketClient<TradeStream>,
}

impl<P> BinanceWebsocketAdaptor<P> for TradeClient where P: SocketPayloadProcess<TradeStreamPayload> + Send + 'static{
    type CLIENT = TradeClient;
    type INPUT = Symbol;
    type OUTPUT = TradeStreamPayload;

    async fn create_client(process: P) -> Self::CLIENT {
        let (client, payload_receiver) =
            WebsocketClient::<TradeStream>::new::<TradeStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(trade_payload_process(trade_stream, process));
        TradeClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(TradeStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|item| TradeStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(TradeStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| TradeStream::new(symbol))
            .collect::<Vec<TradeStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client.get_all_subscribers().iter().map(|item|item.get_symbol()).collect()
    }
}

pub(crate) async fn trade_payload_process<P>(
    trade_response_stream: TradeResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<TradeStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use env_logger::Builder;
    use std::time::Duration;
    use tokio::time::sleep;
    #[tokio::test]
    async fn test_trade() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut trade_client = BinanceMarketWebsocketClient::trade().await;

        trade_client.subscribe_item(Symbol::new("ARKUSDT")).await;

        sleep(Duration::from_secs(10)).await;

        trade_client.subscribe_item(Symbol::new("FILUSDT")).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        trade_client.close().await;

        sleep(Duration::from_millis(1000000)).await;
    }
}
