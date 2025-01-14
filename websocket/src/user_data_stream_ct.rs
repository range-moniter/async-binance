use client::stream::adaptor::BinanceWebsocketAdaptor;
use crate::userdata::types::event_type::UserDataEventPayload;
use client::stream::payload::default_payload_output_func;
use client::stream::stream::{DefaultStreamPayloadProcess, SocketPayloadProcess};
use crate::userdata::user_data_socket::UserDataClient;

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

        UserDataClient::create_client(process).await
    }
}
