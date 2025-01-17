use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::SocketPayloadProcess;
use crate::market::coin_future::agg_trade_ct::UsdFutureAggTradeClient;
use crate::market::types::agg_trade::AggTradeStreamPayload;

pub struct BinanceUsdFutureMarketWebsocketClient;

impl BinanceUsdFutureMarketWebsocketClient {

    pub async fn agg_trade<P>(process: P) -> UsdFutureAggTradeClient
    where
        P: SocketPayloadProcess<AggTradeStreamPayload> + Send + 'static,
    {
        UsdFutureAggTradeClient::create_client(process).await
    }


}
