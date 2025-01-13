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
pub struct DefaultStreamPayloadProcess<F, I>
where
    F: FnMut(BinanceResult<SocketPayloadActor<I>>) + Send,
{
    func: F,
    _phantom: PhantomData<I>,
}

impl<F, I> DefaultStreamPayloadProcess<F, I>
where
    F: FnMut(BinanceResult<SocketPayloadActor<I>>) + Send,
    I: DeserializeOwned + Send + Debug + 'static,
{
    pub fn new(func: F) -> Self {
        DefaultStreamPayloadProcess {
            func,
            _phantom: Default::default(),
        }
    }
    pub fn call(&mut self, param: BinanceResult<SocketPayloadActor<I>>) {
        (self.func)(param)
    }
}

#[async_trait]
impl<I, F> SocketPayloadProcess<I> for DefaultStreamPayloadProcess<F, I>
where
    F: FnMut(BinanceResult<SocketPayloadActor<I>>) + Send,
    I: DeserializeOwned + Send + Debug + 'static,
{
    async fn process(
        &mut self,
        mut stream: Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<I>>> + Send>>,
    ) {
        while let Some(data) = stream.next().await {
            self.call(data);
        }
    }
}
