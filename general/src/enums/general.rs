use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum ContingencyType {
    OCO,
    OTO
}
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum AllocationType {
    SOR
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum WorkflowType {
    EXCHANGE,
    SOR,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum FutureTimeInForce {
    GTC,
    IOC,
    FOK,
    GTX,
    GTD
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum RateLimiterInterval {
    SECOND,
    MINUTE,
    DAY
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum STPModel {
    NONE,
    #[serde(rename= "EXPIRE_MAKER")]
    ExpireMaker,
    #[serde(rename= "EXPIRE_TAKER")]
    ExpireTaker,
    #[serde(rename= "EXPIRE_BOTH")]
    ExpireBoth,
}