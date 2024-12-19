use crate::stream::stream::StreamNameFormat;
use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::mpsc;

// websocket operator
#[derive(Debug)]
pub enum SocketItemChangeActor<I> {
    Add(Vec<I>),
    Remove(Vec<I>),
    Close,
}

pub struct SocketActorHandle<I> {
    socket_item_change_sender: mpsc::Sender<SocketItemChangeActor<I>>,
}

impl<I> SocketActorHandle<I> {
    pub fn new(
        socket_item_change_sender: mpsc::Sender<SocketItemChangeActor<I>>,
    ) -> Self {
        SocketActorHandle {
            socket_item_change_sender,
        }
    }
}

impl<I> SocketActorHandle<I>
where
    I: StreamNameFormat + Clone + Hash + Eq + Send,
{
    pub async fn add_or_remove_socket_item(&mut self, socket_item: SocketItemChangeActor<I>) {
        self.socket_item_change_sender
            .send(socket_item)
            .await
            .unwrap();
    }

    pub async fn send_close(self) {
        self.socket_item_change_sender
            .send(SocketItemChangeActor::Close)
            .await
            .expect("send close websocket signal failed");
    }
}
