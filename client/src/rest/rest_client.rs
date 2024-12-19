use async_trait::async_trait;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tower::{Service, ServiceBuilder};
use tower::timeout::{Timeout, TimeoutLayer};
use general::result::BinanceResult;
use crate::rest::body::RequestBody;
use crate::rest::client::{BinanceClient, BinanceClientAction};
use crate::rest::config::Config;
use crate::rest::extension::RequestExtension;
use crate::rest::layer::authorization::Authorization;
use crate::rest::layer::authorization::layer::AuthorizationLayer;
use crate::rest::layer::rate::layer::WeightRateLimitLayer;
use crate::rest::layer::rate::WeightRateLimiter;

pub type BinanceRestClient = Timeout<WeightRateLimiter<Authorization<Client<HttpsConnector<HttpConnector>, RequestBody>>>>;

impl BinanceClient for BinanceRestClient {
    type Client = BinanceRestClient;
    fn build_client(config: Config) -> Self::Client {
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http2()
            .build();
        let client = Client::builder(TokioExecutor::new())
            .http2_only(true)
            .build(connector);
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(config.request_timeout()))
            .layer(WeightRateLimitLayer::new(config.weight_window_config().window_size(), config.weight_window_config().window_weight()))
            .layer(AuthorizationLayer::default())
            .service(client)
    }
}
#[async_trait]
impl BinanceClientAction for BinanceRestClient {
    async fn get<I, O>(&self, request: I, path: &str, domain: &str, extension: Vec<RequestExtension>) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync ,
        O: DeserializeOwned + Send
    {
        let request = Self::build_get_request(request, path, domain, extension);
        let resp = self.clone().call(request).await.unwrap();
        Self::deserialize_response_body(resp).await
    }

    async fn get_multiple<I, O>(&self, request: I, path: &str, domain: &str, extension: Vec<RequestExtension>) -> BinanceResult<Vec<O>>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send
    {
        let request = Self::build_get_request(request, path, domain, extension);
        let resp = self.clone().call(request).await.unwrap();
        Self::deserialize_response_body(resp).await
    }

    async fn post<I, O>(&self, request: I, path: &str, domain: &str, extension: Vec<RequestExtension>) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send
    {
        let request = Self::build_post_request(request, path, domain, extension);
        let resp = self.clone().call(request).await.unwrap();
        Self::deserialize_response_body(resp).await
    }

    async fn put<I, O>(&self, request: I, path: &str, domain: &str, extension: Vec<RequestExtension>) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send
    {
        let request = Self::build_put_request(request, path, domain, extension);
        let resp = self.clone().call(request).await.unwrap();
        Self::deserialize_response_body(resp).await
    }

    async fn delete<I, O>(&self, request: I, path: &str, domain: &str, extension: Vec<RequestExtension>) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send
    {
        let request = Self::build_delete_request(request, path, domain, extension);
        let resp = self.clone().call(request).await.unwrap();
        Self::deserialize_response_body(resp).await
    }


}
