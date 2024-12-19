mod future;
pub(crate) mod layer;
use crate::rest::body::RequestBody;
use crate::rest::extension::RequestExtension;
use crate::rest::layer::rate::future::RateFuture;
use hyper::Request;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::SystemTime;
use tower::{BoxError, Service};

#[derive(Debug, Clone)]
pub struct WeightWindow {
    weight_window: Arc<Mutex<(u16, u64)>>,
    basic_weight: u16,
    window_size: u64,
}

impl WeightWindow {
    pub fn new(weight: u16, window_edge: u64, window_size: u64) -> Self {
        WeightWindow {
            weight_window: Arc::new(Mutex::new((weight, window_edge))),
            basic_weight: weight,
            window_size,
        }
    }

    pub fn check_weight(&mut self, weight: u16) -> bool {
        let mut guard = self.weight_window.lock().unwrap();
        let (pool_weight, window_edge) = guard.clone();
        let current = current_minutes();
        if current > window_edge {
            *guard = (
                self.basic_weight - weight,
                calculate_window_edge(self.window_size),
            );
            true
        } else {
            if pool_weight >= weight {
                *guard = (pool_weight - weight, window_edge);
                true
            } else {
                false
            }
        }
    }
}

fn current_minutes() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 60
}

pub(crate) fn calculate_window_edge(window_range: u64) -> u64 {
    current_minutes() + window_range
}
#[derive(Clone, Debug)]
pub struct WeightRateLimiter<S> {
    inner: S,
    weight_window: WeightWindow,
}

impl<S> WeightRateLimiter<S> {
    pub(crate) fn new(inner: S, window_size: u64, weight: u16) -> Self {
        WeightRateLimiter {
            inner,
            weight_window: WeightWindow::new(
                weight,
                calculate_window_edge(window_size),
                window_size,
            ),
        }
    }
}

impl<S> Service<Request<RequestBody>> for WeightRateLimiter<S>
where
    S: Service<Request<RequestBody>>,
    S::Error: Into<BoxError>
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = RateFuture<S::Future>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<RequestBody>) -> Self::Future {
        let weight = RequestExtension::explain_request_weight(req.extensions());
        let condition = if let Some(weight) = weight {
            if self.weight_window.check_weight(weight) {
                true
            } else {
                false
            }
        } else {
            true
        };
        RateFuture::new(self.inner.call(req), condition)
    }
}
