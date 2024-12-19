use std::pin::Pin;
use futures_util::Stream;
use crate::market::types::book_depth::{BookDepthStream, BookDepthStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use general::enums::level::Level;
use general::result::BinanceResult;
use general::symbol::Symbol;

pub type BookDepthResponseStream = Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<BookDepthStreamPayload>>> + Send>>;

pub struct BookDepthClient {
    websocket_client: WebsocketClient<BookDepthStream>,
}


impl BookDepthClient {
    pub fn new(websocket_client: WebsocketClient<BookDepthStream>) -> Self {
        Self { websocket_client}
    }

    pub async fn subscribe(&mut self, symbol: Symbol, level: Level) {
        self.websocket_client.subscribe_single(BookDepthStream::new(symbol, level)).await.unwrap();
    }


    pub async fn subscribe_multi(&mut self, params: Vec<(Symbol,Level)>) {
        let params = params.into_iter()
            .map(|(symbol,level)| BookDepthStream::new(symbol, level))
            .collect::<Vec<_>>();
        self.websocket_client.subscribe_multiple(params).await.unwrap()
    }

    pub async fn unsubscribe(&mut self, symbol: Symbol, level: Level) {
        self.websocket_client.unsubscribe_single(BookDepthStream::new(symbol,level)).await.unwrap();
    }

    pub async fn unsubscribe_multi(&mut self, symbols: Vec<(Symbol,Level)>) {
        let params = symbols.into_iter()
            .map(|(symbol,level)| BookDepthStream::new(symbol,level))
            .collect::<Vec<BookDepthStream>>();
        self.websocket_client.unsubscribe_multiple(params).await.unwrap();
    }

    pub async fn close(self) {
        self.websocket_client.close().await;
    }
}

pub(crate) async fn book_depth_payload_process<P>(trade_response_stream: BookDepthResponseStream,
                                                  mut processor: P)
where
    P: SocketPayloadProcess<BookDepthStreamPayload> + Send + 'static,
{
    processor.process(trade_response_stream).await;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::time::sleep;
    use env_logger::Builder;
    use crate::market_socket_ct::BinanceMarketWebsocketClient;
    use super::*;
    #[tokio::test]
    async fn test_average_price() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut book_depth_client = BinanceMarketWebsocketClient::book_depth().await;

        book_depth_client.subscribe(Symbol::new("ARKUSDT"),Level::L1).await;

        sleep(Duration::from_secs(15)).await;

        book_depth_client.subscribe(Symbol::new("FILUSDT"), Level::L1).await;

        sleep(Duration::from_secs(20)).await;

        println!("send close message");

        book_depth_client.close().await;

        sleep(Duration::from_secs(200)).await;
    }
}