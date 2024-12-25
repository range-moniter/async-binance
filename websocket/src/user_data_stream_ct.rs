use crate::userdata::types::event_type::UserDataEventPayload;
use crate::userdata::user_data_socket::{user_data_payload_process, UserDataClient};
use client::stream::client::WebsocketClient;
use client::stream::payload::default_payload_output_func;
use client::stream::stream::{DefaultStreamPayloadProcess, SocketPayloadProcess};
use crate::userdata::types::listen_key::UserDataStream;

pub struct BinanceUserdataWebsocketClient;

impl BinanceUserdataWebsocketClient {
    pub async fn userdata_client() -> UserDataClient {
        let process = DefaultStreamPayloadProcess::new(default_payload_output_func);
        Self::userdata_client_with_process(process).await
    }

    pub async fn userdata_client_with_process<P>(process: P) -> UserDataClient
    where
        P: SocketPayloadProcess<UserDataEventPayload> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<UserDataStream>::new::<UserDataEventPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(user_data_payload_process(trade_stream, process));
        UserDataClient::new(client)
    }
}
