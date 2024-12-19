use general::error::ApplicationError;
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::BoxError;

#[pin_project]
pub struct RateFuture<S> {
    #[pin]
    inner: S,
    condition: bool,
}

impl<S> RateFuture<S> {
    pub fn new(inner: S, condition: bool) -> Self {
        RateFuture { inner, condition }
    }
}



impl<S, R, E> Future for RateFuture<S>
where
    S: Future<Output = Result<R, E>>,
    E: Into<BoxError>,
{
    type Output = Result<R, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.condition {
            let this = self.project();
            match this.inner.poll(cx) {
                Poll::Ready(v) => Poll::Ready(v.map_err(Into::into)),
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(Err(ApplicationError::new("quoted rate limit".to_string()).into()))
        }
    }
}
