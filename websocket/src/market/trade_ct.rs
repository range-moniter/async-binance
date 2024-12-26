use crate::SocketOperator;
use crate::market::types::trade::{TradeStream, TradeStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use general::symbol::Symbol;
use std::collections::HashSet;
use std::pin::Pin;

pub type TradeResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<TradeStreamPayload>>> + Send>>;

pub struct TradeClient {
    websocket_client: WebsocketClient<TradeStream>,
}

impl TradeClient {
    pub fn new(websocket_client: WebsocketClient<TradeStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_trade(&mut self, symbol: Symbol) {
        self.websocket_client
            .subscribe_single(TradeStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn subscribe_trades(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|item| TradeStream::new(item))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(params)
            .await
            .unwrap()
    }

    pub async fn unsubscribe_trade(&mut self, symbol: Symbol) {
        self.websocket_client
            .unsubscribe_single(TradeStream::new(symbol))
            .await
            .unwrap();
    }

    pub async fn unsubscribe_trades(&mut self, symbols: Vec<Symbol>) {
        let params = symbols
            .into_iter()
            .map(|symbol| TradeStream::new(symbol))
            .collect::<Vec<TradeStream>>();
        self.websocket_client
            .unsubscribe_multiple(params)
            .await
            .unwrap();
    }

    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}
impl SocketOperator<TradeStream> for TradeClient {
    async fn close(self) {
        self.close().await
    }

    async fn subscribe_with_entity(&mut self, item: TradeStream) {
        self.websocket_client.subscribe_single(item).await.unwrap()
    }

    async fn subscribe_with_entities(&mut self, items: Vec<TradeStream>) {
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entity(&mut self, item: TradeStream) {
        self.websocket_client
            .unsubscribe_single(item)
            .await
            .unwrap()
    }

    async fn unsubscribe_with_entities(&mut self, items: Vec<TradeStream>) {
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap()
    }

    fn get_all_entities(&self) -> HashSet<TradeStream> {
        self.websocket_client.get_all_subscribers()
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

        trade_client.subscribe_trade(Symbol::new("ARKUSDT")).await;

        sleep(Duration::from_secs(10)).await;

        trade_client.subscribe_trade(Symbol::new("FILUSDT")).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        trade_client.close().await;

        sleep(Duration::from_millis(1000000)).await;
    }
}
