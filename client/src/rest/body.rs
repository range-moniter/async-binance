use bytes::Bytes;
use http_body_util::{Empty, Full};
use hyper::body::{Body, Frame};
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use pin_project::pin_project;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct EmptyRequestData;

#[derive(Debug, Deserialize, Default)]
pub struct EmptyResponseData{}

#[pin_project(project = RequestProj)]
pub enum RequestBody {
    Empty(#[pin] Empty<Bytes>),
    Bytes(#[pin] Full<Bytes>),
}

impl Body for RequestBody {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.project() {
            RequestProj::Empty(body) => body.poll_frame(cx),
            RequestProj::Bytes(body) => body.poll_frame(cx),
        }
    }
}
