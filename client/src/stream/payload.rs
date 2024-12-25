use serde::de::DeserializeOwned;
use std::fmt::Debug;
use serde::Deserialize;
use general::result::BinanceResult;

#[derive(Debug)]
pub enum SocketPayloadActor<O>
{
    Payload(O),
}

impl<O> SocketPayloadActor<O>
where
    O: DeserializeOwned + Send + 'static + Debug,
{
    pub fn payload(self) -> O {
        match self {
            SocketPayloadActor::Payload(payload) => payload,
        }
    }
}

pub fn default_payload_output_func<I>(item: BinanceResult<SocketPayloadActor<I>>)
where
    I: DeserializeOwned + Send + Debug + 'static,
{
    match item {
        Ok(item) => {
            log::info!("Received data: {:?}", item.payload());
        }
        Err(e) => {
            log::error!("Accept socket payload error: error message is: {}", e);
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct SocketOperationResp {
    pub result: Option<String>,
    pub id: u64,
}