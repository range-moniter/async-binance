use crate::market::types::symbol_mini_ticker::{SymbolMiniTickerPayload, SymbolMiniTickerStream};
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::pin::Pin;

pub type MiniTickerResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<SymbolMiniTickerPayload>>> + Send>>;

pub struct SymbolMiniTickerClient {
    websocket_client: WebsocketClient<SymbolMiniTickerStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for SymbolMiniTickerClient {
    type CLIENT = SymbolMiniTickerClient;
    type INPUT = Symbol;
    type OUTPUT = SymbolMiniTickerPayload;

    async fn create_client<P>(process: P, uri: &str) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolMiniTickerStream>::new_with_uri::<SymbolMiniTickerPayload>(uri).await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_mini_ticker_payload_process(trade_stream, process));
        SymbolMiniTickerClient {
            websocket_client: client,
        }
    }


    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(SymbolMiniTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolMiniTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(SymbolMiniTickerStream::new(input))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let params = input
            .into_iter()
            .map(|symbol| SymbolMiniTickerStream::new(symbol))
            .collect::<Vec<_>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| item.get_symbol())
            .collect()
    }
}

pub(crate) async fn symbol_mini_ticker_payload_process<P>(
    trade_response_stream: MiniTickerResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::stream::stream::DefaultStreamPayloadProcess;
    use env_logger::Builder;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::spot_market_socket_ct::BinanceSpotMarketWebsocketClient;

    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut symbol_mini_ticker_client =
            BinanceSpotMarketWebsocketClient::symbol_mini_ticker(DefaultStreamPayloadProcess::new()).await;

        symbol_mini_ticker_client
            .subscribe_item(Symbol::new("ARKUSDT"))
            .await;

        sleep(Duration::from_secs(15)).await;

        symbol_mini_ticker_client
            .subscribe_item(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        symbol_mini_ticker_client
            .subscribe_item(Symbol::new("FILUSDT"))
            .await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        symbol_mini_ticker_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}
