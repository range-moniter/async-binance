use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::stream::DefaultStreamPayloadProcess;
use env_logger::Builder;
use general::enums::interval::Interval;
use general::enums::level::Level;
use general::enums::timezone::Timezone;
use general::enums::window_size::WindowSize;
use general::symbol::Symbol;
use std::time::Duration;
use tokio::time::sleep;
use websocket::market::types::symbol_mini_ticker::TotalSymbolMiniTickerStream;
use websocket::market::types::symbol_ticker::TotalSymbolTickerStream;
use websocket::spot_market_socket_ct::BinanceSpotMarketWebsocketClient;

#[tokio::test]
async fn spot_agg_trade_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::agg_trade(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_trade_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::trade(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_kline_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::kline(DefaultStreamPayloadProcess::new()).await;
    client
        .subscribe_item((
            Symbol::new("ETHUSDT"),
            Interval::Second1,
            Some(Timezone::UTC8),
        ))
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_individual_symbol_mini_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut client =
        BinanceSpotMarketWebsocketClient::symbol_mini_ticker(DefaultStreamPayloadProcess::new())
            .await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_total_symbol_mini_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceSpotMarketWebsocketClient::symbol_mini_ticker_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client
        .subscribe_item(TotalSymbolMiniTickerStream::default())
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_individual_symbol_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::symbol_ticker(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_total_symbol_ticker() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::symbol_ticker_total(DefaultStreamPayloadProcess::new())
            .await;
    client
        .subscribe_item(TotalSymbolTickerStream::default())
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_symbol_rolling_window_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::symbol_rolling_ticker(DefaultStreamPayloadProcess::new())
            .await;
    client
        .subscribe_item((Symbol::new("ETHUSDT"), WindowSize::OneHour))
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_total_symbol_rolling_window() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client = BinanceSpotMarketWebsocketClient::symbol_rolling_ticker_total(
        DefaultStreamPayloadProcess::new(),
    )
    .await;
    client.subscribe_item(WindowSize::OneHour).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_symbol_book_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::symbol_book_ticker(DefaultStreamPayloadProcess::new())
            .await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_average_price_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::average_price(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item(Symbol::new("ETHUSDT")).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_partial_book_depth_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::partial_book_depth(DefaultStreamPayloadProcess::new())
            .await;
    client
        .subscribe_item((Symbol::new("ETHUSDT"), Level::L1, None))
        .await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}

#[tokio::test]
async fn spot_diff_book_depth_test() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let mut client =
        BinanceSpotMarketWebsocketClient::diff_book_depth(DefaultStreamPayloadProcess::new()).await;
    client.subscribe_item((Symbol::new("ETHUSDT"), None)).await;
    sleep(Duration::from_secs(2)).await;
    client.close().await;
    sleep(Duration::from_secs(2)).await;
    print!("over");
}
