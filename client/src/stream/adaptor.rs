use async_trait::async_trait;
use crate::stream::stream::SocketPayloadProcess;

#[async_trait]
pub trait BinanceWebsocketAdaptor<P: SocketPayloadProcess<Self::OUTPUT>> {
    type CLIENT;
    type INPUT;
    type OUTPUT;
    async fn create_client(process: P) -> Self::CLIENT;
    async fn close(self);
    async fn subscribe_item(&mut self, input: Self::INPUT);
    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>);
    async fn unsubscribe_item(&mut self, input: Self::INPUT);
    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>);
    fn get_subscribe_items(&self) -> Vec<Self::INPUT>;
}
