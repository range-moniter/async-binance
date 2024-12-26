use std::collections::HashSet;
pub use client::stream;
pub use general::*;
pub mod market;
pub mod market_socket_ct;
pub mod user_data_stream_ct;
pub mod userdata;

pub trait SocketOperator<I> {
    async fn close(self);
    async fn subscribe_with_entity(&mut self, item: I);
    async fn subscribe_with_entities(&mut self, items: Vec<I>);
    async fn unsubscribe_with_entity(&mut self, item: I);
    async fn unsubscribe_with_entities(&mut self, items: Vec<I>);
    fn get_all_entities(&self) -> HashSet<I>;
}