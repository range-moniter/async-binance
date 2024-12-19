use std::pin::Pin;
use futures_util::Stream;
use crate::market::types::kline::{KlineStream, KlineStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use general::enums::interval::Interval;
use general::result::BinanceResult;
use general::symbol::Symbol;

pub type KlineResponseStream = Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<KlineStreamPayload>>> + Send>>;

pub struct KlineClient {
    websocket_client: WebsocketClient<KlineStream>,
}


impl KlineClient {
    pub fn new(websocket_client: WebsocketClient<KlineStream>) -> Self {
        Self { websocket_client }
    }

    pub async fn subscribe_kline(&mut self, symbol: Symbol, interval: Interval) {
        self.websocket_client.subscribe_single(KlineStream::new(symbol, interval)).await.unwrap();
    }


    pub async fn subscribe_kline_multi(&mut self, params: Vec<(Symbol, Interval)>) {
        let params = params.into_iter()
            .map(|(symbol, interval)| KlineStream::new(symbol, interval))
            .collect::<Vec<_>>();
        self.websocket_client.subscribe_multiple(params).await.unwrap()
    }

    pub async fn unsubscribe_kline(&mut self, symbol: Symbol, interval: Interval) {
        self.websocket_client.unsubscribe_single(KlineStream::new(symbol, interval)).await.unwrap();
    }

    pub async fn unsubscribe_kline_multi(&mut self, symbols: Vec<(Symbol, Interval)>) {
        let params = symbols.into_iter()
            .map(|(symbol, interval)| KlineStream::new(symbol, interval))
            .collect::<Vec<KlineStream>>();
        self.websocket_client.unsubscribe_multiple(params).await.unwrap();
    }

    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}

pub(crate) async fn kline_payload_process<P>(trade_response_stream: KlineResponseStream,
                                             mut processor: P)
where
    P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use env_logger::Builder;
    use log::Level;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use super::*;
    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut kline_client = BinanceMarketWebsocketClient::kline().await;

        kline_client.subscribe_kline(Symbol::new("ARKUSDT"), Interval::Minute1).await;

        sleep(Duration::from_secs(60)).await;

        kline_client.subscribe_kline(Symbol::new("FILUSDT"), Interval::Second1).await;

        sleep(Duration::from_secs(100)).await;

        kline_client.subscribe_kline(Symbol::new("ETHUSDT"), Interval::Second1).await;

        sleep(Duration::from_secs(50)).await;

        kline_client.unsubscribe_kline(Symbol::new("FILUSDT"), Interval::Second1).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        kline_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}