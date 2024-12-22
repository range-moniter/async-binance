use client::rest::body::{EmptyRequestData, EmptyResponseData};
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use client::rest::layer::authorization::types::{AuthType, Certificate};
use general::result::BinanceResult;
use crate::userdata::types::listen_key::UserDataStream;

pub struct UserDataRestClient<T> {
    client: T,
    domain: String,
}

impl<T> UserDataRestClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> Self {
        UserDataRestClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }

    async fn create_listen_key(&self, certificate: Certificate) -> BinanceResult<UserDataStream> {
        self.client
            .post(
                EmptyRequestData::default(),
                "/api/v3/userDataStream",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserStream, 1, certificate),
            )
            .await
    }

    async fn put_listen_key(
        &self,
        listen_key: UserDataStream,
        certificate: Certificate,
    ) -> BinanceResult<EmptyResponseData> {
        self.client
            .put(
                listen_key,
                "/api/v3/userDataStream",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserStream, 1, certificate),
            )
            .await
    }

    async fn delete_listen_key(
        &self,
        listen_key: UserDataStream,
        certificate: Certificate,
    ) -> BinanceResult<EmptyResponseData> {
        self.client
            .delete(
                listen_key,
                "/api/v3/userDataStream",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserStream, 1, certificate),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::rest::config::Config;
    use client::rest::rest_client::BinanceRestClient;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: UserDataRestClient<BinanceRestClient> =
            UserDataRestClient::new(BinanceRestClient::build_client(Config::new_default()));
        static ref CERTIFICATE: Certificate = Certificate::new(
            "",
            ""
        );
    }

    #[tokio::test]
    async fn test_create_listen_key() {
        let resp = CLIENT.create_listen_key(CERTIFICATE.clone()).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_put_listen_key() {
        let resp = CLIENT
            .put_listen_key(
                UserDataStream::new("QNHdn1cKG53xux4kH36CmUeoZlyanoodZA70OfVm7RYXPJ8iGeaac65F0Lvb"),
                CERTIFICATE.clone(),
            )
            .await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_delete_listen_key() {
        let resp = CLIENT
            .delete_listen_key(
                UserDataStream::new("QNHdn1cKG53xux4kH36CmUeoZlyanoodZA70OfVm7RYXPJ8iGeaac65F0Lvb"),
                CERTIFICATE.clone(),
            )
            .await;
        println!("{:?}", resp);
    }
}
