use std::fmt::Debug;
use std::pin::Pin;
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use general::result::BinanceResult;
use crate::stream::payload::SocketPayloadActor;

pub trait StreamNameFormat {
    fn stream_name(&self) -> String;
}

#[async_trait]
pub trait SocketPayloadProcess<I: DeserializeOwned + Send + Debug + 'static> {
    async fn process(&mut self, stream: Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<I>>> + Send>>);
}

#[derive(Debug, Default)]
pub struct DefaultStreamPayloadProcess;

#[async_trait]
impl<I> SocketPayloadProcess<I> for DefaultStreamPayloadProcess
where
    I: DeserializeOwned + Send + 'static + Debug,
{
    async fn process(&mut self, mut stream: Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<I>>> + Send>>) {
        while let Some(data) = stream.next().await {
            match data {
                Ok(item) => {
                    log::info!("Received data: {:?}", item.payload());
                }
                Err(e) => {
                    log::error!("Accept socket payload error: error message is: {}", e);
                }
            }
        }
    }
}