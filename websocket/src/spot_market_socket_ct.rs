use crate::market::agg_trade_ct::AggTradeClient;
use crate::market::avg_price_ct::AveragePriceClient;
use crate::market::book_depth_ct::BookDepthClient;
use crate::market::book_ticker_ct::SymbolBookTickerClient;
use crate::market::depth_ct::DepthClient;
use crate::market::kline_ct::KlineClient;
use crate::market::mini_ticker_ct::SymbolMiniTickerClient;
use crate::market::mini_ticker_total_ct::TotalSymbolMiniTickerClient;
use crate::market::rolling_ct::SymbolRollingClient;
use crate::market::rolling_total_ct::TotalSymbolRollingClient;
use crate::market::ticker_ct::SymbolTickerClient;
use crate::market::ticker_total_ct::TotalSymbolTickerClient;
use crate::market::trade_ct::TradeClient;
use crate::market::types::agg_trade::AggTradeStreamPayload;
use crate::market::types::average_price::AveragePricePayload;
use crate::market::types::book_depth::BookDepthStreamPayload;
use crate::market::types::depth::DepthStreamPayload;
use crate::market::types::kline::KlineStreamPayload;
use crate::market::types::symbol_book_ticker::SymbolBookTickerPayload;
use crate::market::types::symbol_mini_ticker::{
    SymbolMiniTickerPayload, TotalSymbolMiniTickerPayload,
};
use crate::market::types::symbol_rolling::{
    SymbolRollingPayload, TotalSymbolRollingPayload,
};
use crate::market::types::symbol_ticker::{SymbolTickerPayload, TotalSymbolTickerPayload};
use crate::market::types::trade::TradeStreamPayload;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::SocketPayloadProcess;

pub struct BinanceSpotMarketWebsocketClient;

const SPOT_SOCKET_URI: &str = "wss://stream.binance.com:9443/ws";
impl BinanceSpotMarketWebsocketClient {
    /// office doc(https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams#aggregate-trade-streams)
    pub async fn trade<P>(process: P) -> TradeClient
    where
        P: SocketPayloadProcess<TradeStreamPayload> + Send + 'static,
    {
        TradeClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn agg_trade<P>(process: P) -> AggTradeClient
    where
        P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
    {
        AggTradeClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn average_price<P>(process: P) -> AveragePriceClient
    where
        P: SocketPayloadProcess<AveragePricePayload> + Send + 'static,
    {
        AveragePriceClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn diff_book_depth<P>(process: P) -> DepthClient
    where
        P: SocketPayloadProcess<DepthStreamPayload> + Send + 'static,
    {
        DepthClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn partial_book_depth<P>(process: P) -> BookDepthClient
    where
        P: SocketPayloadProcess<BookDepthStreamPayload> + Send + 'static,
    {
        BookDepthClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn kline<P>(process: P) -> KlineClient
    where
        P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
    {
        KlineClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_book_ticker<P>(process: P) -> SymbolBookTickerClient
    where
        P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
    {
        SymbolBookTickerClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_mini_ticker<P>(process: P) -> SymbolMiniTickerClient
    where
        P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
    {
        SymbolMiniTickerClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_mini_ticker_total<P>(process: P) -> TotalSymbolMiniTickerClient
    where
        P: SocketPayloadProcess<TotalSymbolMiniTickerPayload> + Send + 'static,
    {
        TotalSymbolMiniTickerClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_rolling_ticker<P>(process: P) -> SymbolRollingClient
    where
        P: SocketPayloadProcess<SymbolRollingPayload> + Send + 'static,
    {
        SymbolRollingClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_rolling_ticker_total<P>(process: P) -> TotalSymbolRollingClient
    where
        P: SocketPayloadProcess<TotalSymbolRollingPayload> + Send + 'static,
    {
        TotalSymbolRollingClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_ticker<P>(process: P) -> SymbolTickerClient
    where
        P: SocketPayloadProcess<SymbolTickerPayload> + Send + 'static,
    {
        SymbolTickerClient::create_client(process, SPOT_SOCKET_URI).await
    }

    pub async fn symbol_ticker_total<P>(process: P) -> TotalSymbolTickerClient
    where
        P: SocketPayloadProcess<TotalSymbolTickerPayload> + Send + 'static,
    {
        TotalSymbolTickerClient::create_client(process, SPOT_SOCKET_URI).await
    }
}
