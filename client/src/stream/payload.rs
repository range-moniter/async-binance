use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde::Deserialize;

#[derive(Debug)]
pub enum SocketPayloadActor<O>
where
    O: DeserializeOwned + Send + 'static + Debug,
{
    Payload(O),
}

impl<O> SocketPayloadActor<O>
where
    O: DeserializeOwned + Send + 'static + Debug,
{
    pub fn payload(&self) -> &O {
        match self {
            &SocketPayloadActor::Payload(ref payload) => payload,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SocketOperationResp {
    pub result: Option<String>,
    pub id: u64,
}