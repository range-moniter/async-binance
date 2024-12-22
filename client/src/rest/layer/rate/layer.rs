use crate::rest::layer::rate::WeightRateLimiter;
use tower::Layer;

pub struct WeightRateLimitLayer {
    time_window: u64,
    limit: u16,
}

impl WeightRateLimitLayer {
    pub const fn new(time_window: u64, limit: u16) -> Self {
        WeightRateLimitLayer { time_window, limit }
    }
}

impl<I> Layer<I> for WeightRateLimitLayer  {
    type Service = WeightRateLimiter<I>;

    fn layer(&self, inner: I) -> Self::Service {
        WeightRateLimiter::new_with_default(inner)
    }
}
