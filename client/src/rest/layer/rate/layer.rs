use crate::rest::layer::rate::WeightRateLimiter;
use tower::Layer;

#[derive(Debug, Default)]
pub struct WeightRateLimitLayer;

impl WeightRateLimitLayer {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<I> Layer<I> for WeightRateLimitLayer  {
    type Service = WeightRateLimiter<I>;

    fn layer(&self, inner: I) -> Self::Service {
        WeightRateLimiter::new_with_default(inner)
    }
}
