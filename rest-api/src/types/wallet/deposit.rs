use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetDepositHistoryReq {
    #[serde(rename = "includeSource")]
    include_source: Option<bool>,
    coin: Option<String>,
    status: Option<u8>,
    #[serde(rename = "startTime")]
    start_time: Option<u64>,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    offset: Option<u64>,
    limit: Option<u32>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u64>,
    #[serde(rename = "txId")]
    tx_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetDepositHistoryResp {
    pub id: String,
    pub amount: BigDecimal,
    pub coin: String,
    pub network: String,
    pub status: u8,
    pub address: String,
    #[serde(rename = "addressTag")]
    pub address_tag: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "insertTime")]
    pub insert_time: u64,
    #[serde(rename = "transferType")]
    pub transfer_type: u64,
    #[serde(rename = "confirmTimes")]
    pub confirm_times: String,
    #[serde(rename = "unlockConfirm")]
    pub unlock_confirm: u64,
    #[serde(rename = "walletType")]
    pub wallet_type: u8,
}

#[derive(Serialize, Debug)]
pub struct GetDepositAddressReq {
    coin: String,
    network: Option<String>,
    amount: Option<BigDecimal>,
    #[serde(rename = "recvWindow")]
    recv_window: Option<u64>,
}
#[derive(Serialize, Deserialize)]
pub struct GetDepositAddressResp {
    pub address: String,
    pub coin: String,
    pub tag: String,
    pub url: String,
}

#[derive(Serialize, Debug)]
pub struct GetDepositAddressWithNetworkReq {
    coin: String,
    network: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetDepositAddressWithNetworkResp {
    pub coin: String,
    pub address: String,
    pub tag: String,
    #[serde(rename = "isDefault")]
    pub is_default: u8,
}