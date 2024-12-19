use std::pin::Pin;
use futures_util::Stream;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use general::result::BinanceResult;
use crate::userdata::types::event_type::UserDataEventPayload;
use crate::userdata::types::listen_key::UserDataStream;

pub type UserDataResponseStream  = Pin<Box<dyn Stream<Item=BinanceResult<SocketPayloadActor<UserDataEventPayload>>> + Send>>;


pub struct UserDataClient {
    websocket_client: WebsocketClient<UserDataStream>,
}

impl UserDataClient {
    pub fn new(websocket_client: WebsocketClient<UserDataStream>) -> Self {
        Self { websocket_client }
    }
    pub async fn sub_user_data_stream(&mut self, listen_key: &str) {
        self.websocket_client.subscribe_single(UserDataStream::new(listen_key)).await.unwrap();
    }
}

pub(crate) async fn user_data_payload_process<P>(user_data_stream: UserDataResponseStream,
                                                     mut processor: P)
where
    P: SocketPayloadProcess<UserDataEventPayload> + Send + 'static,
{
    processor.process(user_data_stream).await;
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use env_logger::Builder;
    use crate::user_data_stream_ct::BinanceUserdataWebsocketClient;
    use super::*;
    #[tokio::test]
    pub async fn test_user_user_data_stream() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();

        let mut client = BinanceUserdataWebsocketClient::userdata_client().await;
        client.sub_user_data_stream("jxSl0enlQguoDW2J3AvP4HEBKVXe4zKkh4PxE5CCMYhLVOCqIJnDjIa8nOxh").await;
        tokio::time::sleep(Duration::from_secs(1000)).await;
    }
}