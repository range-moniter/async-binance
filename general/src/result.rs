use crate::error::SdkError;

pub type BinanceResult<T> = Result<T, SdkError>;
