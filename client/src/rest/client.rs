use crate::rest::body::RequestBody;
use crate::rest::config::Config;
use crate::rest::extension::RequestExtension;
use async_trait::async_trait;
use bytes::Bytes;
use general::error::SdkError;
use general::result::BinanceResult;
use http_body::Body;
use http_body_util::{BodyExt, Empty};
use hyper::{Request, Response, Uri};
use serde::Serialize;
use serde::de::DeserializeOwned;

const HTTP_SCHEME: &str = "https";

pub trait BinanceClient {
    type Client;
    fn build_client(config: Config) -> Self::Client;
}

#[async_trait]
pub trait BinanceClientAction {
    async fn get<I, O>(
        &self,
        request: Option<I>,
        path: &str,
        domain: &str,
        extension: Vec<RequestExtension>,
    ) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send;

    async fn get_multiple<I, O>(
        &self,
        request: Option<I>,
        path: &str,
        domain: &str,
        extension: Vec<RequestExtension>,
    ) -> BinanceResult<Vec<O>>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send;

    async fn post<I, O>(
        &self,
        request: Option<I>,
        path: &str,
        domain: &str,
        extension: Vec<RequestExtension>,
    ) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send;

    async fn put<I, O>(
        &self,
        request: Option<I>,
        path: &str,
        domain: &str,
        extension: Vec<RequestExtension>,
    ) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send;

    async fn delete<I, O>(
        &self,
        request: Option<I>,
        path: &str,
        domain: &str,
        extension: Vec<RequestExtension>,
    ) -> BinanceResult<O>
    where
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send;

    fn build_get_request<I>(
        request: Option<I>,
        path: &str,
        domain: &str,
        mut extension: Vec<RequestExtension>,
    ) -> Request<RequestBody>
    where
        I: Serialize,
    {
        let uri = Self::build_uri(path, domain);
        if request.is_some() {
            let params = serde_urlencoded::to_string(&request)
                .unwrap_or_else(|err| panic!("serialize error: {}", err));
            extension.push(RequestExtension::Param(params));
        }
        Request::get(uri)
            .extension(extension)
            .body(RequestBody::Empty(Empty::<Bytes>::new()))
            .unwrap()
    }

    fn build_delete_request<I>(
        request: Option<I>,
        path: &str,
        domain: &str,
        mut extension: Vec<RequestExtension>,
    ) -> Request<RequestBody>
    where
        I: Serialize,
    {
        let uri = Self::build_uri(path, domain);
        Self::add_body_extension(request, &mut extension);
        Request::delete(uri)
            .extension(extension)
            .body(RequestBody::Empty(Empty::<Bytes>::new()))
            .unwrap()
    }

    fn add_body_extension<I>(request: Option<I>, extension: &mut Vec<RequestExtension>)
    where
        I: Serialize,
    {
        if request.is_some() {
            let params = serde_urlencoded::to_string(request)
                .unwrap_or_else(|err| panic!("serialize error: {}", err));
            extension.push(RequestExtension::Body(params));
        }
    }

    fn build_post_request<I>(
        request: Option<I>,
        path: &str,
        domain: &str,
        mut extension: Vec<RequestExtension>,
    ) -> Request<RequestBody>
    where
        I: Serialize,
    {
        let uri = Self::build_uri(path, domain);
        Self::add_body_extension(request, &mut extension);
        Request::post(uri)
            .extension(extension)
            .body(RequestBody::Empty(Empty::<Bytes>::new()))
            .unwrap()
    }

    fn build_put_request<I>(
        request: Option<I>,
        path: &str,
        domain: &str,
        mut extension: Vec<RequestExtension>,
    ) -> Request<RequestBody>
    where
        I: Serialize,
    {
        let uri = Self::build_uri(path, domain);
        Self::add_body_extension(request, &mut extension);
        Request::put(uri)
            .extension(extension)
            .body(RequestBody::Empty(Empty::<Bytes>::new()))
            .unwrap()
    }

    fn build_uri(path: &str, domain: &str) -> Uri {
        Uri::builder()
            .scheme(HTTP_SCHEME)
            .authority(domain)
            .path_and_query(path)
            .build()
            .unwrap()
    }

    async fn deserialize_response_body<R, O>(mut resp: Response<R>) -> BinanceResult<O>
    where
        R: Body<Data = Bytes> + Send + Unpin + 'static,
        R::Error: std::error::Error + Send + Sync + 'static,
        O: DeserializeOwned,
    {
        let mut body = vec![];
        let body_stream = resp.body_mut();
        while let Some(chunk) = body_stream.frame().await {
            match chunk {
                Ok(chunk) => body.extend(chunk.into_data().unwrap()),
                Err(err) => {
                    return Err(SdkError::ResponseBodyFrameError(format!("{:#?}", err)));
                }
            }
        }
        log::debug!("response body: {:?}", String::from_utf8_lossy(&body));
        if resp.status().is_success() {
            Ok(serde_json::from_slice::<O>(&body)?)
        } else {
            Err(SdkError::BinanceError(serde_json::from_slice(&body)?))
        }
    }
}
