mod future;
pub(crate) mod layer;
pub mod types;

use crate::rest::body::RequestBody;
use crate::rest::extension::RequestExtension;
use crate::rest::layer::rate::future::RateFuture;
use hyper::Request;
use std::task::{Context, Poll};
use tower::{BoxError, Service};
use crate::rest::layer::rate::types::ip_handle::IpWeightHandle;
use crate::rest::layer::rate::types::order_handle::OrderHandle;
use crate::rest::layer::rate::types::uid_handle::UidWeightHandle;

#[derive(Clone, Debug)]
pub struct WeightRateLimiter<S> {
    inner: S,
    ip_weight_rate_handle: IpWeightHandle,
    uid_weight_handle: UidWeightHandle,
    order_handle: OrderHandle,
}

impl<S> WeightRateLimiter<S>
{
    pub(crate) fn new_with_default(inner: S) -> Self {
        WeightRateLimiter {
            inner,
            ip_weight_rate_handle: IpWeightHandle::new_default(),
            uid_weight_handle: UidWeightHandle::new_with_default(),
            order_handle: OrderHandle::new_with_default(),
        }
    }
}

impl<S> Service<Request<RequestBody>> for WeightRateLimiter<S>
where
    S: Service<Request<RequestBody>>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = RateFuture<S::Future>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<RequestBody>) -> Self::Future {
        let weight = RequestExtension::explain_request_weight(req.extensions()).unwrap();
        let ip_rate = RequestExtension::explain_request_ip_rate(req.extensions());
        let uid_rate = RequestExtension::explain_request_uid_rate(req.extensions());
        let order_rate = RequestExtension::explain_request_order_rate(req.extensions());
        let condition = {
            let uid_check = self.uid_weight_handle.available(weight, uid_rate);
            let order_check = self.order_handle.available(order_rate);
            let ip_check = self.ip_weight_rate_handle.available(weight, ip_rate);
            if uid_check && order_check && ip_check {
                true
            } else {
                false
            }
        };

        RateFuture::new(self.inner.call(req), condition)
    }
}

