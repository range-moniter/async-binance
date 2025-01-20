use crate::market::agg_trade_ct::AggTradeClient;
use crate::market::book_ticker_ct::SymbolBookTickerClient;
use crate::market::book_ticker_total_ct::TotalSymbolBookTickerClient;
use crate::market::continuous_kline_ct::ContinuousKlineClient;
use crate::market::kline_ct::KlineClient;
use crate::market::liquidation_order_ct::LiquidationOrderClient;
use crate::market::liquidation_order_total_ct::TotalLiquidationOrderClient;
use crate::market::mark_price_ct::MarkPriceClient;
use crate::market::mark_price_total_ct::MarkPriceTotalClient;
use crate::market::mini_ticker_ct::SymbolMiniTickerClient;
use crate::market::mini_ticker_total_ct::TotalSymbolMiniTickerClient;
use crate::market::rolling_ct::SymbolRollingClient;
use crate::market::ticker_ct::SymbolTickerClient;
use crate::market::ticker_total_ct::TotalSymbolTickerClient;
use crate::market::types::agg_trade::AggTradeStreamPayload;
use crate::market::types::continuous_kline::ContinuousKlineStreamPayload;
use crate::market::types::kline::KlineStreamPayload;
use crate::market::types::liquidation_order::LiquidationOrderStreamPayload;
use crate::market::types::mark_price::{MarkPriceStreamPayload, TotalMarkPriceStreamPayload};
use crate::market::types::symbol_book_ticker::SymbolBookTickerPayload;
use crate::market::types::symbol_mini_ticker::{
    SymbolMiniTickerPayload, TotalSymbolMiniTickerPayload,
};
use crate::market::types::symbol_rolling::SymbolRollingPayload;
use crate::market::types::symbol_ticker::{SymbolTickerPayload, TotalSymbolTickerPayload};
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::SocketPayloadProcess;

const USD_FUTURE_SOCKET_URI: &str = "wss://fstream.binance.com/ws";

pub struct BinanceUsdFutureMarketWebsocketClient;

impl BinanceUsdFutureMarketWebsocketClient {
    pub async fn agg_trade<P>(process: P) -> AggTradeClient
    where
        P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
    {
        AggTradeClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn mark_price<P>(process: P) -> MarkPriceClient
    where
        P: SocketPayloadProcess<MarkPriceStreamPayload> + Send + 'static,
    {
        MarkPriceClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn mark_price_total<P>(process: P) -> MarkPriceTotalClient
    where
        P: SocketPayloadProcess<TotalMarkPriceStreamPayload> + Send + 'static,
    {
        MarkPriceTotalClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn kline<P>(process: P) -> KlineClient
    where
        P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
    {
        KlineClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn continuous_kline<P>(process: P) -> ContinuousKlineClient
    where
        P: SocketPayloadProcess<ContinuousKlineStreamPayload> + Send + 'static,
    {
        ContinuousKlineClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }
    pub async fn symbol_book_ticker<P>(process: P) -> SymbolBookTickerClient
    where
        P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
    {
        SymbolBookTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_book_ticker_total<P>(process: P) -> TotalSymbolBookTickerClient
    where
        P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
    {
        TotalSymbolBookTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_mini_ticker<P>(process: P) -> SymbolMiniTickerClient
    where
        P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
    {
        SymbolMiniTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_mini_ticker_total<P>(process: P) -> TotalSymbolMiniTickerClient
    where
        P: SocketPayloadProcess<TotalSymbolMiniTickerPayload> + Send + 'static,
    {
        TotalSymbolMiniTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_rolling_ticker<P>(process: P) -> SymbolRollingClient
    where
        P: SocketPayloadProcess<SymbolRollingPayload> + Send + 'static,
    {
        SymbolRollingClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_ticker<P>(process: P) -> SymbolTickerClient
    where
        P: SocketPayloadProcess<SymbolTickerPayload> + Send + 'static,
    {
        SymbolTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn symbol_ticker_total<P>(process: P) -> TotalSymbolTickerClient
    where
        P: SocketPayloadProcess<TotalSymbolTickerPayload> + Send + 'static,
    {
        TotalSymbolTickerClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn liquidation_order<P>(process: P) -> LiquidationOrderClient
    where
        P: SocketPayloadProcess<LiquidationOrderStreamPayload> + Send + 'static,
    {
        LiquidationOrderClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }

    pub async fn liquidation_order_total<P>(process: P) -> TotalLiquidationOrderClient
    where
        P: SocketPayloadProcess<LiquidationOrderStreamPayload> + Send + 'static,
    {
        TotalLiquidationOrderClient::create_client(process, USD_FUTURE_SOCKET_URI).await
    }
}
