use crate::stream::actor::{SocketActorHandle, SocketItemChangeActor};
use crate::stream::payload::SocketPayloadActor;
use crate::stream::socket::{SocketReceiverState, SocketSenderState, stream};
use crate::stream::stream::StreamNameFormat;
use general::result::BinanceResult;
use serde::de::DeserializeOwned;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::mpsc::{UnboundedReceiver, channel, unbounded_channel};
use tokio_tungstenite::tungstenite::Message;

pub struct WebsocketClient<I> {
    socket_item: HashSet<I>,
    socket_actor_handle: SocketActorHandle<I>,
}

impl<I> WebsocketClient<I> {
    pub async fn new<O>() -> (
        WebsocketClient<I>,
        UnboundedReceiver<BinanceResult<SocketPayloadActor<O>>>,
    )
    where
        I: StreamNameFormat + Clone + Hash + Eq + Send + 'static,
        O: DeserializeOwned + Send + 'static + Debug,
    {
        let (item_sender, item_receiver) = channel::<SocketItemChangeActor<I>>(64);
        let socket_actor_handle = SocketActorHandle::new(item_sender);
        let (socket_health_sender, socket_health_receiver) = channel::<Message>(1);
        let (payload_sender, payload_receiver) =
            unbounded_channel::<BinanceResult<SocketPayloadActor<O>>>();
        let socket_sender_state =
            SocketSenderState::new(item_receiver, socket_health_receiver).await;
        let socket_receiver_state = SocketReceiverState::new(payload_sender, socket_health_sender);
        tokio::spawn(stream(socket_sender_state, socket_receiver_state));
        let client = WebsocketClient {
            socket_item: HashSet::new(),
            socket_actor_handle,
        };
        (client, payload_receiver)
    }

    pub async fn new_with_uri<O>(uri: &str) -> (
        WebsocketClient<I>,
        UnboundedReceiver<BinanceResult<SocketPayloadActor<O>>>,
    )
    where
        I: StreamNameFormat + Clone + Hash + Eq + Send + 'static,
        O: DeserializeOwned + Send + 'static + Debug,
    {
        let (item_sender, item_receiver) = channel::<SocketItemChangeActor<I>>(64);
        let socket_actor_handle = SocketActorHandle::new(item_sender);
        let (socket_health_sender, socket_health_receiver) = channel::<Message>(1);
        let (payload_sender, payload_receiver) =
            unbounded_channel::<BinanceResult<SocketPayloadActor<O>>>();
        let socket_sender_state =
            SocketSenderState::new_with_uri(uri, item_receiver, socket_health_receiver).await;
        let socket_receiver_state = SocketReceiverState::new(payload_sender, socket_health_sender);
        tokio::spawn(stream(socket_sender_state, socket_receiver_state));
        let client = WebsocketClient {
            socket_item: HashSet::new(),
            socket_actor_handle,
        };
        (client, payload_receiver)
    }
}

impl<I> WebsocketClient<I>
where
    I: Hash + Eq + Clone + StreamNameFormat + Send + 'static,
{
    pub async fn subscribe_single(&mut self, param: I) -> BinanceResult<()> {
        self.subscribe_multiple(vec![param]).await
    }

    pub async fn subscribe_multiple(&mut self, params: Vec<I>) -> BinanceResult<()> {
        let effect_items = self.add_socket_items(params);
        self.socket_actor_handle
            .add_or_remove_socket_item(SocketItemChangeActor::Add(effect_items))
            .await;
        Ok(())
    }

    pub async fn unsubscribe_single(&mut self, param: I) -> BinanceResult<()> {
        self.unsubscribe_multiple(vec![param]).await
    }

    pub async fn unsubscribe_multiple(&mut self, params: Vec<I>) -> BinanceResult<()> {
        let effect_items = self.remove_socket_items(params);
        self.socket_actor_handle
            .add_or_remove_socket_item(SocketItemChangeActor::Remove(effect_items))
            .await;
        Ok(())
    }
    pub fn get_all_subscribers(&self) -> &HashSet<I> {
        &self.socket_item
    }
    pub async fn close(self) {
        self.socket_actor_handle.send_close().await;
    }
    fn add_socket_items(&mut self, items: Vec<I>) -> Vec<I> {
        let new_items = items
            .into_iter()
            .filter(|item| !self.socket_item.contains(item))
            .clone()
            .collect::<Vec<_>>();
        new_items.iter().for_each(|item| {
            self.socket_item.insert(item.clone());
        });
        new_items
    }

    fn remove_socket_items(&mut self, items: Vec<I>) -> Vec<I> {
        let new_items = items
            .into_iter()
            .filter(|item| self.socket_item.contains(item))
            .clone()
            .collect::<Vec<_>>();
        new_items.iter().for_each(|item| {
            self.socket_item.remove(item);
        });
        new_items
    }
}
