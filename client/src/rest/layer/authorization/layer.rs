use tower::Layer;
use crate::rest::layer::authorization::Authorization;

#[derive(Debug, Default)]
pub struct AuthorizationLayer;

impl <I> Layer<I> for AuthorizationLayer  {
    type Service = Authorization<I>;

    fn layer(&self, inner: I) -> Self::Service {
        Authorization::new(inner)
    }
}