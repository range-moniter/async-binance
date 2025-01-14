use crate::userdata::types::event_type::UserDataEventPayload;
use crate::userdata::types::listen_key::UserDataStream;
use async_trait::async_trait;
use client::stream::adaptor::BinanceWebsocketAdaptor;
use client::stream::client::WebsocketClient;
use client::stream::payload::SocketPayloadActor;
use client::stream::stream::SocketPayloadProcess;
use futures_util::Stream;
use general::result::BinanceResult;
use std::pin::Pin;

pub type UserDataResponseStream =
    Pin<Box<dyn Stream<Item = BinanceResult<SocketPayloadActor<UserDataEventPayload>>> + Send>>;

pub struct UserDataClient {
    websocket_client: WebsocketClient<UserDataStream>,
}
#[async_trait]
impl BinanceWebsocketAdaptor for UserDataClient {
    type CLIENT = UserDataClient;
    type INPUT = String;
    type OUTPUT = UserDataEventPayload;

    async fn create_client<P>(process: P) -> Self::CLIENT
    where
        P: SocketPayloadProcess<Self::OUTPUT> + Send + 'static,
    {
        let (client, payload_receiver) =
            WebsocketClient::<UserDataStream>::new::<UserDataEventPayload>().await;
        let trade_stream = Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(
            payload_receiver,
        ));
        tokio::spawn(user_data_payload_process(trade_stream, process));
        UserDataClient {
            websocket_client: client,
        }
    }

    async fn close(self) {
        self.websocket_client.close().await
    }

    async fn subscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .subscribe_single(UserDataStream::new(input.as_str()))
            .await
            .unwrap();
    }

    async fn subscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let items = input
            .iter()
            .map(|item| UserDataStream::new(item.as_str()))
            .collect::<Vec<_>>();
        self.websocket_client
            .subscribe_multiple(items)
            .await
            .unwrap();
    }

    async fn unsubscribe_item(&mut self, input: Self::INPUT) {
        self.websocket_client
            .unsubscribe_single(UserDataStream::new(input.as_str()))
            .await
            .unwrap();
    }

    async fn unsubscribe_items(&mut self, input: Vec<Self::INPUT>) {
        let items = input
            .iter()
            .map(|item| UserDataStream::new(item.as_str()))
            .collect::<Vec<_>>();
        self.websocket_client
            .unsubscribe_multiple(items)
            .await
            .unwrap();
    }

    fn get_subscribe_items(&self) -> Vec<Self::INPUT> {
        self.websocket_client
            .get_all_subscribers()
            .iter()
            .map(|item| item.listen_key.clone())
            .collect::<Vec<_>>()
    }
}

pub(crate) async fn user_data_payload_process<P>(
    user_data_stream: UserDataResponseStream,
    mut processor: P,
) where
    P: SocketPayloadProcess<UserDataEventPayload> + Send + 'static,
{
    processor.process(user_data_stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_data_stream_ct::BinanceUserdataWebsocketClient;
    use client::stream::stream::DefaultStreamPayloadProcess;
    use env_logger::Builder;
    use std::time::Duration;

    // #[tokio::test]
    // pub async fn test_user_user_data_stream() {
    //     Builder::from_default_env()
    //         .filter(None, log::LevelFilter::Debug)
    //         .init();
    //
    //     let mut client = BinanceUserdataWebsocketClient::userdata_client(DefaultStreamPayloadProcess::default()).await;
    //     client
    //         .subscribe_item("jxSl0enlQguoDW2J3AvP4HEBKVXe4zKkh4PxE5CCMYhLVOCqIJnDjIa8nOxh".to_string())
    //         .await;
    //     tokio::time::sleep(Duration::from_secs(1000)).await;
    // }
}
