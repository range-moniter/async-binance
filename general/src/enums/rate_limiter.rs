use serde::{Deserialize, Serialize};



#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum RateLimitUnit {
    SECOND,
    MINUTE,
    HOUR,
    DAY,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum RateLimitType {
    #[serde(rename = "REQUEST_WEIGHT")]
    RequestWeight,
    ORDERS,
    #[serde(rename = "RAW_REQUESTS")]
    RawRequests,
}


#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct RateLimiter {
    #[serde(rename = "rateLimitType")]
    pub rate_limit_type: RateLimitType,
    #[serde(rename = "interval")]
    pub interval: RateLimitUnit,
    #[serde(rename = "intervalNum")]
    pub interval_num: u64,
    #[serde(rename = "limit")]
    pub limit: u32,
}