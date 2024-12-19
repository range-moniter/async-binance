use crate::rest::body::RequestBody;
use crate::rest::extension::RequestExtension;
use crate::rest::layer::authorization::sign::sign;
use crate::rest::layer::authorization::types::{AuthType, Certificate};
use http_body_util::{Empty, Full};
use hyper::http::{Extensions, HeaderValue};
use hyper::{Request, Uri};
use std::task::{Context, Poll};
use std::time::SystemTime;
use bytes::Bytes;
use tower::Service;

pub mod layer;
mod sign;
pub mod types;

#[derive(Debug, Clone)]
pub struct Authorization<S> {
    inner: S,
}

impl<S> Authorization<S> {
    pub fn new(inner: S) -> Self {
        Authorization { inner }
    }
    pub fn get_inner_ref(&self) -> &S {
        &self.inner
    }
}

impl<S> Service<Request<RequestBody>> for Authorization<S>
where
    S: Service<Request<RequestBody>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
       self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<RequestBody>) -> Self::Future {
        let (auth_type, weight, certificate) =
            RequestExtension::explain_auth_type(req.extensions());
        let request_body = RequestExtension::explain_request_body(req.extensions());
        let request_param = RequestExtension::explain_request_params(req.extensions());
        match auth_type {
            Some(auth_type) => match auth_type {
                AuthType::None => {
                    self.inner.call(rebuild_request(req, request_param, request_body))
                },
                AuthType::UserStream | AuthType::MarketData => {
                    let cert = get_certificate(certificate.clone());
                    req.headers_mut().append(
                        "X-MBX-APIKEY",
                        HeaderValue::from_str(cert.api_key()).unwrap(),
                    );
                    self.inner.call(rebuild_request(req, request_param, request_body))
                }
                AuthType::Trade | AuthType::UserData => {
                    let cert = get_certificate(certificate.clone());
                    let window_size = get_window_size(weight);
                    req.headers_mut().append(
                        "X-MBX-APIKEY",
                        HeaderValue::from_str(cert.api_key()).unwrap(),
                    );
                    let time_and_window_size = get_time_and_window_params(window_size);
                    let new_body = if let Some(body) = request_body {
                        let payload = format!("{}&{}", body, &time_and_window_size);
                        let sign = sign(payload, cert.secret_key())
                            .unwrap_or_else(|err| panic!("sign error: {:?}", err));
                        let body = format!(
                            "{}&{}&signature={}",
                            body,
                            time_and_window_size,
                            sign
                        );
                        println!("{body}");
                        Some(body)
                    } else {
                        None
                    };
                    let new_params = if let Some(param) = request_param {
                        let payload = format!("{}&{}", param, &time_and_window_size);
                        let sign = sign(payload, cert.secret_key())
                            .unwrap_or_else(|err| panic!("sign error: {:?}", err));
                        let param = format!(
                            "{}&{}&signature={}",
                            param.clone(),
                            time_and_window_size,
                            sign
                        );
                        Some(param)
                    } else {
                        None
                    };
                    let request = rebuild_request(req, new_params, new_body);
                    self.inner.call(request)
                }
            },
            None => {
                let request = rebuild_request(req, request_param, request_body);
                self.inner.call(request)
            },
        }
    }
}

fn get_time_and_window_params(window_size: u16) -> String {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let append = format!("timestamp={}&recvWindow={}", current_time, window_size);
    append
}
fn get_window_size(window_size: Option<u16>) -> u16 {
    match window_size {
        Some(window_size) => {
            if window_size > 60000 {
                60000
            } else if window_size < 5000 {
                5000
            } else {
                window_size
            }
        }
        None => 5000,
    }
}
fn get_certificate(cert: Option<Certificate>) -> Certificate {
    cert.unwrap_or_else(|| panic!("certificate not set;"))
}

fn rebuild_request(
    req: Request<RequestBody>,
    new_params: Option<String>,
    new_body: Option<String>,
) -> Request<RequestBody> {
    let (parts, _) = req.into_parts();
    let method = parts.method;
    let headers = parts.headers;
    let extensions = build_new_extension(parts.extensions);
    let uri = build_new_params(parts.uri, new_params);
    let body = build_new_body(new_body);
    let mut request = Request::builder()
        .method(method)
        .uri(uri)
        .extension(extensions)
        .body(body)
        .unwrap();
    request.headers_mut().extend(headers);
    request
}

fn build_new_extension(extension: Extensions) -> Vec<RequestExtension> {
    let ext_val = extension.get::<Vec<RequestExtension>>().unwrap();
    ext_val
        .into_iter()
        .filter(|item| !matches!(item, RequestExtension::Weight(_)))
        .map(|item|item.clone())
        .collect::<Vec<_>>()
}

fn build_new_params(uri: Uri, params: Option<String>) -> Uri {
    match params {
        None => uri,
        Some(param) => {
            let mut uri_parts = uri.into_parts();
            let path_and_query = uri_parts.path_and_query.unwrap();
            let mut url = path_and_query.as_str().to_string();
            if !url.contains('?') {
                url.push('?');
            } else if !url.ends_with('?') {
                url.push('&');
            }
            url.push_str(param.as_str());
            uri_parts.path_and_query = Some(url.parse().expect("Failed to parse modified URI"));
            Uri::from_parts(uri_parts).expect("Failed to parse URI")
        }
    }
}

fn build_new_body(body: Option<String>) -> RequestBody {
    match body {
        Some(body) => {
            RequestBody::Bytes(Full::from(body.into_bytes()))
        }
        _ => {
            RequestBody::Empty(Empty::<Bytes>::new())
        }
    }
}