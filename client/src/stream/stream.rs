use crate::stream::payload::SocketPayloadActor;
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use general::result::BinanceResult;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::pin::Pin;

pub trait StreamNameFormat {
    fn stream_name(&self) -> String;
}

#[async_trait]
pub trait SocketPayloadProcess<I> {
    async fn process(
        &mut self,
        stream: Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<I>>> + Send>>,
    );
}

#[derive(Debug)]
pub struct DefaultStreamPayloadProcess<I>{
    _phantom: PhantomData<I>,
}

impl<I> DefaultStreamPayloadProcess<I> {
    pub fn new() -> Self{
        DefaultStreamPayloadProcess {
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<I> SocketPayloadProcess<I> for DefaultStreamPayloadProcess<I>
where
    I: DeserializeOwned + Send + Debug + 'static,
{
    async fn process(
        &mut self,
        mut stream: Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<I>>> + Send>>,
    ) {
        while let Some(data) = stream.next().await {
            match data {
                Ok(item) => {
                    log::info!("Received data: {:?}", item);
                }
                Err(e) => {
                    log::error!("Accept socket payload error: error message is: {}", e);
                }
            }
        }
    }
}
