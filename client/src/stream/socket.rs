use crate::stream::actor::SocketItemChangeActor;
use crate::stream::payload::{SocketOperationResp, SocketPayloadActor};
use crate::stream::stream::StreamNameFormat;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use general::error::map_deserialization_error;
use general::result::BinanceResult;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedSender};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

lazy_static! {
    static ref GLOBAL_ATOMIC_ID: AtomicU64 = AtomicU64::new(1);
}

enum SocketMethod {
    Subscribe,
    Unsubscribe,
}

impl SocketMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            SocketMethod::Subscribe => "SUBSCRIBE",
            SocketMethod::Unsubscribe => "UNSUBSCRIBE",
        }
    }
}

pub struct SocketReceiverState<O>
where
    O: DeserializeOwned + Send + 'static + Debug
{
    payload_sender: UnboundedSender<BinanceResult<SocketPayloadActor<O>>>,
    socket_health_sender: Sender<Message>,
}
impl<O> SocketReceiverState<O>
where
    O: DeserializeOwned + Send + 'static + Debug,
{
    pub fn new(
        payload_sender: UnboundedSender<BinanceResult<SocketPayloadActor<O>>>,
        socket_health_sender: Sender<Message>,
    ) -> Self {
        SocketReceiverState {
            payload_sender,
            socket_health_sender,
        }
    }
}
pub struct SocketSenderState<I> {
    id: u64,
    socket_item_change_receiver: Receiver<SocketItemChangeActor<I>>,
    websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    socket_health_receiver: Receiver<Message>,
}

impl<I> SocketSenderState<I> {
    pub async fn new(
        socket_item_receiver: Receiver<SocketItemChangeActor<I>>,
        socket_health_receiver: Receiver<Message>,
    ) -> Self {
        let stream = Self::new_async_connect_default().await;
        SocketSenderState {
            id: GLOBAL_ATOMIC_ID.fetch_add(1, Ordering::SeqCst),
            socket_item_change_receiver: socket_item_receiver,
            websocket_stream: stream,
            socket_health_receiver,
        }
    }

    pub async fn new_with_uri(
        uri: &str,
        socket_item_receiver: Receiver<SocketItemChangeActor<I>>,
        socket_health_receiver: Receiver<Message>,
    ) -> Self {
        let stream = Self::new_async_connect(uri).await;
        SocketSenderState {
            id: GLOBAL_ATOMIC_ID.fetch_add(1, Ordering::SeqCst),
            socket_item_change_receiver: socket_item_receiver,
            websocket_stream: stream,
            socket_health_receiver,
        }
    }

    pub async fn new_async_connect_default() -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        Self::new_async_connect("wss://stream.binance.com:9443/ws").await
    }

    pub async fn new_async_connect(uri: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        log::info!("Connecting to {}...", uri);
        let (socket, response) = connect_async(uri).await.expect("Failed to connect");
        log::debug!("WebSocket response status: {}", response.status());
        for (ref header, value) in response.headers() {
            log::debug!("WebSocket response header: {}={:#?}", header, value);
        }
        socket
    }
}

fn build_socket_message(
    id: u64,
    method: &str,
    params: impl IntoIterator<Item = String>,
) -> Message {
    let mut params_str: String = params
        .into_iter()
        .map(|param| format!("\"{}\"", param))
        .collect::<Vec<String>>()
        .join(",");

    if !params_str.is_empty() {
        params_str = format!("\"params\": [{params}],", params = params_str)
    };
    let s = format!(
        "{{\"method\":\"{method}\",{params}\"id\":{id}}}",
        method = method,
        params = params_str,
        id = id
    );
    let message = Message::Text(s);
    log::debug!("build socket message: {}", message);
    message
}

// socket_state: Binance websocket connection ,It will be divided into separate read and write socket connections.
// payload_sender： when accept socket data, it will send those data to consumer, and every client have a consumer handle (P: SocketPayloadProcess), and there is
//                                              a default processor(DefaultStreamPayloadProcess: it will print all message on console);
// socket_health_sender: socket heartbeat sender
pub async fn stream<I, O>(
    socket_sender_state: SocketSenderState<I>,
    socket_receiver_state: SocketReceiverState<O>,
) where
    I: StreamNameFormat + Clone + Hash + Eq + Send,
    O: DeserializeOwned + 'static + Send + Debug,
{
    let (writer_socket, reader_socket) = socket_sender_state.websocket_stream.split();
    let socket_item_receiver = socket_sender_state.socket_item_change_receiver;
    let socket_health_receiver = socket_sender_state.socket_health_receiver;
    tokio::select! {
        _ = run_socket_receive(reader_socket, socket_receiver_state.payload_sender, socket_receiver_state.socket_health_sender) => {}
        _ = run_socket_sender(writer_socket, socket_item_receiver, socket_sender_state.id, socket_health_receiver) => {}
    }
}

// writer_socket: The actual socket connection for sending messages to Binance.
// socket_receiver: Accept subscription or unsubscription message , when accept a message, wrapper the message and send to binance by writer_socket,
// id: socket id
// socket_health_receiver: Accept pong message, and send to Binance by writer_socket.
async fn run_socket_sender<I>(
    mut writer_socket: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut socket_receiver: Receiver<SocketItemChangeActor<I>>,
    id: u64,
    mut socket_health_receiver: Receiver<Message>,
) where
    I: StreamNameFormat + Clone + Hash + Eq + Send,
{
    loop {
        let message = tokio::select! {
            operator_message = async {
                let message = if let Some(message) = socket_receiver.recv().await {
                    let msg = match message {
                        SocketItemChangeActor::Add(data) => {
                            let params = data.iter()
                                    .map(|req_item| req_item.stream_name())
                                    .collect::<Vec<_>>();
                            log::debug!("Accept add sub message, the message data is: {:?}", params);
                            Some(build_socket_message(id, SocketMethod::Subscribe.as_str(), params))
                        }
                        SocketItemChangeActor::Remove(data) => {
                            let params = data.iter()
                                            .map(|req_item| req_item.stream_name())
                                            .collect::<Vec<_>>();
                            log::debug!("Accept remove sub message, the message data is: {:?}", params);
                            Some(build_socket_message(id, SocketMethod::Unsubscribe.as_str(), params))
                        }
                        SocketItemChangeActor::Close => {
                            log::info!("Accept socket close signal, the socket will close: {}", id);
                            Some(Message::Close(None))
                        }
                    };
                    msg
                } else {
                    None
                };
                message
            } => operator_message,
            pong_message = async {
                let msg = socket_health_receiver.recv().await.unwrap();
                log::debug!("Accept ping message, will send pong to binance.");
                Some(msg)
            } => pong_message,
        };
        log::info!("send socket message: {:?}", message);
        match message {
            None => {}
            Some(item) => {
                let break_signal = match &item {
                    Message::Close(_) => Some(1),
                    _ => None,
                };
                log::debug!("Send message: {}", item);
                writer_socket
                    .send(item)
                    .await
                    .expect("Failed to send message");
                if let Some(_) = break_signal {
                    log::info!("Socket close signal, the socket will close: {}", id);
                    break;
                }
            }
        }
    }
}
// 1. reader_socket: receive binance data. including subscription data and heartbeat data, when receive heartbeat data,
//  use socket_health_sender send pong message to socket_health_receiver, and then send pong message to binance
// 2. payload_sender: when receive binance subscription data , It will send the data to the actual consumers through the payload_sender.
// 3. socket_health_sender: whenever a ping message is received, a pong message will be send to the socket_health_receiver
async fn run_socket_receive<O>(
    mut reader_socket: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    payload_sender: UnboundedSender<BinanceResult<SocketPayloadActor<O>>>,
    socket_health_sender: Sender<Message>,
) where
    O: DeserializeOwned + 'static + Send + Debug,
{
    while let Some(message) = reader_socket.next().await {
        match message {
            Ok(message) => match message {
                Message::Ping(data) => {
                    log::debug!("Received ping message: {}", String::from_utf8_lossy(&data));
                    socket_health_sender
                        .send(Message::Pong(data))
                        .await
                        .expect("Failed send pong to receiver");
                }
                Message::Close(_) => {
                    log::info!("Accept socket close message, socket will close");
                    payload_sender.send(Ok(SocketPayloadActor::Close(1))).unwrap();
                    break;
                }
                Message::Text(data) => {
                    log::debug!("Received text message: {}", data);
                    let response = serde_json::from_str::<O>(data.as_str());
                    let response = match response {
                        Ok(response) => Some(Ok(SocketPayloadActor::Payload(response))),
                        Err(e) => {
                            match serde_json::from_str::<SocketOperationResp>(data.as_str()) {
                                Ok(operator) => {
                                    log::info!("Operator success response: {:?}", operator);
                                    None
                                }
                                _ => Some(Err(map_deserialization_error(e, data.as_bytes()))),
                            }
                        }
                    };
                    match response {
                        Some(resp) => {
                            if let Err(e) = payload_sender.send(resp) {
                                log::error!("Failed to send response: {}, body={}", e, data);
                            }
                        }
                        None => {}
                    }
                }
                Message::Pong(data) => {
                    log::warn!("Received pong message: {}", String::from_utf8_lossy(&data));
                }
                Message::Binary(data) => {
                    log::warn!(
                        "Received binary message: {}",
                        String::from_utf8_lossy(&data)
                    );
                }
                Message::Frame(frame) => {
                    log::warn!("Received frame message: {}", frame);
                }
            },
            Err(err) => {
                log::error!("websocket_accept_error: {}, the streaming will break;", err);
                break;
            }
        }
    }
}
