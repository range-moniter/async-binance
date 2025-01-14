use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde::Deserialize;

#[derive(Debug)]
pub enum SocketPayloadActor<O>
{
    Payload(O),
    Close(u8),
}

impl<O> SocketPayloadActor<O>
where
    O: DeserializeOwned + Send + 'static + Debug,
{
    pub fn payload(self) -> O {
        match self {
            SocketPayloadActor::Payload(payload) => payload,
            _ => panic!("other payload cannot support"),
        }
    }
}



#[derive(Debug, Deserialize)]
pub struct SocketOperationResp {
    pub result: Option<String>,
    pub id: u64,
}