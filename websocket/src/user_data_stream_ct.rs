use client::stream::adaptor::BinanceWebsocketAdaptor;
use crate::userdata::types::event_type::UserDataEventPayload;
use client::stream::stream::SocketPayloadProcess;
use crate::userdata::user_data_socket::UserDataClient;

pub struct BinanceUserdataWebsocketClient;

impl BinanceUserdataWebsocketClient {
    pub async fn userdata_client<P>(process: P) -> UserDataClient
    where
        P: SocketPayloadProcess<UserDataEventPayload> + Send + 'static,
    {
        UserDataClient::create_client(process).await
    }
}
