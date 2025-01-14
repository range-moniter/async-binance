use crate::stream::stream::SocketPayloadProcess;
use async_trait::async_trait;

#[async_trait]
pub trait BinanceWebsocketAdaptor {
    type CLIENT;
    type INPUT;
    type OUTPUT;
    async fn create_client<P>(process: P) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static ;
    async fn close(self);
    async fn subscribe_item(&mut self, input: Self::INPUT);
    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>);
    async fn unsubscribe_item(&mut self, input: Self::INPUT);
    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>);
    fn get_subscribe_items(&self) -> Vec<Self::INPUT>;
}
