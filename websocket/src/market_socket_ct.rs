use crate::market::agg_trade_ct::{agg_trade_payload_process, AggTradeClient};
use crate::market::avg_price_ct::{avg_price_payload_process, AveragePriceClient};
use crate::market::book_depth_ct::{book_depth_payload_process, BookDepthClient};
use crate::market::book_ticker_ct::{symbol_book_ticker_payload_process, SymbolBookTickerClient};
use crate::market::depth_ct::{depth_payload_process, DepthClient};
use crate::market::kline_ct::{kline_payload_process, KlineClient};
use crate::market::mini_ticker_ct::{symbol_mini_ticker_payload_process, SymbolMiniTickerClient};
use crate::market::rolling_ct::{symbol_rolling_payload_process, SymbolRollingClient};
use crate::market::ticker_ct::{symbol_ticker_payload_process, SymbolTickerClient};
use crate::market::trade_ct::{trade_payload_process, TradeClient};
use crate::market::types::agg_trade::{AggTradeStream, AggTradeStreamPayload};
use crate::market::types::average_price::{AveragePricePayload, AveragePriceStream};
use crate::market::types::book_depth::{BookDepthStream, BookDepthStreamPayload};
use crate::market::types::depth::{DepthStream, DepthStreamPayload};
use crate::market::types::kline::{KlineStream, KlineStreamPayload};
use crate::market::types::symbol_book_ticker::{SymbolBookTickerPayload, SymbolBookTickerStream};
use crate::market::types::symbol_mini_ticker::{SymbolMiniTickerPayload, SymbolMiniTickerStream};
use crate::market::types::symbol_rolling::{SymbolRollingPayload, SymbolRollingWindowStream};
use crate::market::types::symbol_ticker::{SymbolTickerPayload, SymbolTickerStream};
use crate::market::types::trade::{TradeStream, TradeStreamPayload};
use client::stream::client::WebsocketClient;
use client::stream::stream::{DefaultStreamPayloadProcess, SocketPayloadProcess};

pub struct BinanceMarketWebsocketClient;

impl BinanceMarketWebsocketClient {
    pub async fn trade() -> TradeClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::trade_with_process(process).await
    }

    pub async fn trade_with_process<P>(process: P) -> TradeClient
    where
        P: SocketPayloadProcess<TradeStreamPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<TradeStream>::new::<TradeStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(trade_payload_process(trade_stream, process));
        TradeClient::new(client)
    }

    pub async fn agg_trade() -> AggTradeClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::agg_trade_with_process(process).await
    }

    pub async fn agg_trade_with_process<P>(process: P) -> AggTradeClient
    where
        P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<AggTradeStream>::new::<AggTradeStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(agg_trade_payload_process(trade_stream, process));
        AggTradeClient::new(client)
    }

    pub async fn average_price() -> AveragePriceClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::average_price_with_process(process).await
    }

    pub async fn average_price_with_process<P>(process: P) -> AveragePriceClient
    where
        P: SocketPayloadProcess<AveragePricePayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<AveragePriceStream>::new::<AveragePricePayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(avg_price_payload_process(trade_stream, process));
        AveragePriceClient::new(client)
    }

    pub async fn book_depth() -> BookDepthClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::book_depth_with_process(process).await
    }

    pub async fn book_depth_with_process<P>(process: P) -> BookDepthClient
    where
        P: SocketPayloadProcess<BookDepthStreamPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<BookDepthStream>::new::<BookDepthStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(book_depth_payload_process(trade_stream, process));
        BookDepthClient::new(client)
    }
    pub async fn depth() -> DepthClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::depth_with_process(process).await
    }

    pub async fn depth_with_process<P>(process: P) -> DepthClient
    where
        P: SocketPayloadProcess<DepthStreamPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<DepthStream>::new::<DepthStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(depth_payload_process(trade_stream, process));
        DepthClient::new(client)
    }

    pub async fn kline() -> KlineClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::kline_with_process(process).await
    }

    pub async fn kline_with_process<P>(process: P) -> KlineClient
    where
        P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<KlineStream>::new::<KlineStreamPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(kline_payload_process(trade_stream, process));
        KlineClient::new(client)
    }

    pub async fn symbol_book_ticker() -> SymbolBookTickerClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::symbol_book_ticker_with_process(process).await
    }

    pub async fn symbol_book_ticker_with_process<P>(process: P) -> SymbolBookTickerClient
    where
        P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolBookTickerStream>::new::<SymbolBookTickerPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_book_ticker_payload_process(trade_stream, process));
        SymbolBookTickerClient::new(client)
    }

    pub async fn symbol_mini_ticker() -> SymbolMiniTickerClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::symbol_mini_ticker_with_process(process).await
    }

    pub async fn symbol_mini_ticker_with_process<P>(process: P) -> SymbolMiniTickerClient
    where
        P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolMiniTickerStream>::new::<SymbolMiniTickerPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_mini_ticker_payload_process(trade_stream, process));
        SymbolMiniTickerClient::new(client)
    }

    pub async fn symbol_rolling_ticker() -> SymbolRollingClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::symbol_rolling_ticker_with_process(process).await
    }

    pub async fn symbol_rolling_ticker_with_process<P>(process: P) -> SymbolRollingClient
    where
        P: SocketPayloadProcess<SymbolRollingPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolRollingWindowStream>::new::<SymbolRollingPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_rolling_payload_process(trade_stream, process));
        SymbolRollingClient::new(client)
    }

    pub async fn symbol_ticker() -> SymbolTickerClient {
        let process = DefaultStreamPayloadProcess::default();
        Self::symbol_ticker_with_process(process).await
    }

    pub async fn symbol_ticker_with_process<P>(process: P) -> SymbolTickerClient
    where
        P: SocketPayloadProcess<SymbolTickerPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<SymbolTickerStream>::new::<SymbolTickerPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(symbol_ticker_payload_process(trade_stream, process));
        SymbolTickerClient::new(client)
    }
}
