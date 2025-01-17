use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::SocketPayloadProcess;
use crate::market::spot::agg_trade_ct::SpotAggTradeClient;
use crate::market::spot::avg_price_ct::SpotAveragePriceClient;
use crate::market::spot::book_depth_ct::SpotBookDepthClient;
use crate::market::spot::book_ticker_ct::SpotSymbolBookTickerClient;
use crate::market::spot::depth_ct::SpotDepthClient;
use crate::market::spot::kline_ct::SpotKlineClient;
use crate::market::spot::mini_ticker_ct::SpotSymbolMiniTickerClient;
use crate::market::spot::rolling_ct::SpotSymbolRollingClient;
use crate::market::spot::ticker_ct::SpotSymbolTickerClient;
use crate::market::spot::trade_ct::SpotTradeClient;
use crate::market::types::agg_trade::AggTradeStreamPayload;
use crate::market::types::average_price::AveragePricePayload;
use crate::market::types::book_depth::BookDepthStreamPayload;
use crate::market::types::depth::DepthStreamPayload;
use crate::market::types::kline::KlineStreamPayload;
use crate::market::types::symbol_book_ticker::SymbolBookTickerPayload;
use crate::market::types::symbol_mini_ticker::SymbolMiniTickerPayload;
use crate::market::types::symbol_rolling::SymbolRollingPayload;
use crate::market::types::symbol_ticker::SymbolTickerPayload;
use crate::market::types::trade::TradeStreamPayload;

pub struct BinanceSpotMarketWebsocketClient;

impl BinanceSpotMarketWebsocketClient {
    pub async fn trade<P>(process: P) -> SpotTradeClient
    where
        P: SocketPayloadProcess<TradeStreamPayload> + Send + 'static,
    {
        SpotTradeClient::create_client(process).await
    }

    pub async fn agg_trade<P>(process: P) -> SpotAggTradeClient
    where
        P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
    {
        SpotAggTradeClient::create_client(process).await
    }


    pub async fn average_price<P>(process: P) -> SpotAveragePriceClient
    where
        P: SocketPayloadProcess<AveragePricePayload> + Send + 'static,
    {
        SpotAveragePriceClient::create_client(process).await
    }

    pub async fn book_depth<P>(process: P) -> SpotBookDepthClient
    where
        P: SocketPayloadProcess<BookDepthStreamPayload> + Send + 'static,
    {

        SpotBookDepthClient::create_client(process).await
    }

    pub async fn depth<P>(process: P) -> SpotDepthClient
    where
        P: SocketPayloadProcess<DepthStreamPayload> + Send + 'static,
    {
        SpotDepthClient::create_client(process).await
    }

    pub async fn kline<P>(process: P) -> SpotKlineClient
    where
        P: SocketPayloadProcess<KlineStreamPayload> + Send + 'static,
    {
        SpotKlineClient::create_client(process).await
    }

    pub async fn symbol_book_ticker<P>(process: P) -> SpotSymbolBookTickerClient
    where
        P: SocketPayloadProcess<SymbolBookTickerPayload> + Send + 'static,
    {
        SpotSymbolBookTickerClient::create_client(process).await
    }

    pub async fn symbol_mini_ticker<P>(process: P) -> SpotSymbolMiniTickerClient
    where
        P: SocketPayloadProcess<SymbolMiniTickerPayload> + Send + 'static,
    {
        SpotSymbolMiniTickerClient::create_client(process).await
    }

    pub async fn symbol_rolling_ticker<P>(process: P) -> SpotSymbolRollingClient
    where
        P: SocketPayloadProcess<SymbolRollingPayload> + Send + 'static,
    {
        SpotSymbolRollingClient::create_client(process).await
    }


    pub async fn symbol_ticker<P>(process: P) -> SpotSymbolTickerClient
    where
        P: SocketPayloadProcess<SymbolTickerPayload> + Send + 'static,
    {
        SpotSymbolTickerClient::create_client(process).await
    }
}
