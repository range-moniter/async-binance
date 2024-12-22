use std::fmt::Debug;
use general::error::SdkError;
use general::result::BinanceResult;

pub(crate) mod ip_handle;
mod window;
pub(crate) mod order_handle;
pub(crate) mod uid_handle;

#[derive(Debug, Copy, Clone)]
pub enum RateType {
    IpWeightRate(bool),
    UidRate(bool, u64),
    OrderRate(u64),
}


impl PartialEq for RateType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RateType::IpWeightRate(_), RateType::IpWeightRate(_)) => true,
            (RateType::UidRate(_, _), RateType::UidRate(_, _)) => true,
            (RateType::OrderRate(_), RateType::OrderRate(_)) => true,
            _ => false,
        }
    }
}


impl RateType {
    pub fn is_sapi(&self) -> bool {
        match self {
            RateType::IpWeightRate(is_sapi) => {
                *is_sapi
            }
            RateType::UidRate(is_sapi, _) => {
                *is_sapi
            }
            _ => false
        }
    }

    pub fn get_uid(&self) -> BinanceResult<u64> {
        match self {
            RateType::IpWeightRate(_) => Err(SdkError::ParameterError("IpWeightRate dont have uid".to_string())),
            RateType::UidRate(_, uid) => { Ok(*uid) }
            RateType::OrderRate(uid) => { Ok(*uid) }
        }
    }
}