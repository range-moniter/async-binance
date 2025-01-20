use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::DefaultStreamPayloadProcess;
use env_logger::Builder;
use general::enums::contract_type::ContractType;
use general::enums::interval::Interval;
use general::enums::level::Level;
use general::enums::speed::Speed;
use general::symbol::Symbol;
use std::time::Duration;
use tokio::time::sleep;
use websocket::market::types::contract_info::ContractInfoStream;
use websocket::market::types::liquidation_order::TotalLiquidationOrderStream;
use websocket::market::types::mark_price::TotalMarkPriceStream;
use websocket::market::types::symbol_book_ticker::TotalSymbolBookTickerStream;
use websocket::market::types::symbol_mini_ticker::TotalSymbolMiniTickerStream;
use websocket::market::types::symbol_ticker::TotalSymbolTickerStream;
use websocket::usd_future_market_socket_ct::BinanceUsdFutureMarketWebsocketClient;

#[tokio::test]
async fn usd_future_test_agg_trade() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::agg_trade(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(Symbol::new("BANUSDT")).await;
    client.subscribe_item(Symbol::new("BBUSDT")).await;
    sleep(Duration::from_millis(10000)).await;
    client.close().await;
    sleep(Duration::from_millis(2000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_mark_price() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::mark_price(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item((Symbol::new("BANUSDT"), true)).await;
    client.subscribe_item((Symbol::new("BBUSDT"), false)).await;
    sleep(Duration::from_millis(10000)).await;
    client.close().await;
    sleep(Duration::from_millis(2000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_mark_all_price() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::mark_price_total(DefaultStreamPayloadProcess::new())
            .await;
    client
        .subscribe_item(TotalMarkPriceStream::new(false))
        .await;
    sleep(Duration::from_millis(5000)).await;
    client.close().await;
    sleep(Duration::from_millis(2000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_kline_candle() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::kline(DefaultStreamPayloadProcess::new()).await;
    client
        .subscribe_item((Symbol::new("BTCUSDT"), Interval::Minute1, None))
        .await;
    sleep(Duration::from_secs(100)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_continuous_contract_kline() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::continuous_kline(DefaultStreamPayloadProcess::new())
            .await;
    client
        .subscribe_item((
            Symbol::new("BTCUSDT"),
            ContractType::Perpetual,
            Interval::Minute1,
        ))
        .await;
    sleep(Duration::from_secs(10)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_individual_symbol_mini_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::symbol_mini_ticker(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client.subscribe_item(Symbol::new("BANUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_symbol_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::symbol_ticker(DefaultStreamPayloadProcess::new())
            .await;
    client.subscribe_item(Symbol::new("BANUSDT")).await;
    sleep(Duration::from_secs(5)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_total_symbol_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::symbol_ticker_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item(TotalSymbolTickerStream::default())
        .await;
    sleep(Duration::from_secs(3)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_total_symbol_mini_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::symbol_mini_ticker_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item(TotalSymbolMiniTickerStream::default())
        .await;
    sleep(Duration::from_secs(3)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_symbol_book_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::symbol_book_ticker(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client.subscribe_item(Symbol::new("BBUSDT")).await;
    sleep(Duration::from_secs(3)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_all_symbol_book_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::symbol_book_ticker_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item(TotalSymbolBookTickerStream::default())
        .await;
    sleep(Duration::from_secs(3)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_liquidation_order() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::liquidation_order(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client.subscribe_item(Symbol::new("USUALUSDT")).await;
    sleep(Duration::from_secs(20)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_total_liquidation_order() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::liquidation_order_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item(TotalLiquidationOrderStream::default())
        .await;
    sleep(Duration::from_secs(20)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over")
}

#[tokio::test]
async fn usd_future_test_partial_book_depth() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::partial_book_depth(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item((Symbol::new("ETHUSDT"), Level::L1, Some(Speed::Ms100)))
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_diff_book_depth() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceUsdFutureMarketWebsocketClient::diff_book_depth(DefaultStreamPayloadProcess::new())
            .await;
    client.subscribe_item((Symbol::new("ETHUSDT"), None)).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}


#[tokio::test]
async fn usd_future_test_composite_index_symbol() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::composite_index_symbol(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client.subscribe_item(Symbol::new("BTCUSDT")).await;
    sleep(Duration::from_secs(30)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}

#[tokio::test]
async fn usd_future_test_contract_info() {
    Builder::from_default_env()
    .filter(None, log::LevelFilter::Info)
    .init();
    let mut client = BinanceUsdFutureMarketWebsocketClient::contract_info(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(ContractInfoStream::default()).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_millis(5000)).await;
    print!("over");
}