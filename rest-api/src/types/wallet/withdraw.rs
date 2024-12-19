use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct CreateWithdrawReq {

    coin: String,
    withdraw_order_id: Option<String>,
    network: Option<String>,
    address: String,
    address_tag: Option<String>,
    amount: BigDecimal,
    transaction_fee_flag: Option<bool>,
    name: Option<String>,
    wallet_type: Option<u64>,
    recv_window: Option<u64>,

}

#[derive(Deserialize, Debug)]
pub struct CreateWithdrawResp {
    pub id: String,
}

#[derive(Serialize, Debug)]
pub struct GetWithdrawHistoryReq {
    coin: Option<String>,
    #[serde(rename="withdrawOrderId")]
    withdraw_order_id: Option<String>,
    status: Option<u8>,
    offset: Option<u32>,
    limit: Option<u32>,
    #[serde(rename="idList")]
    id_list: Option<String>,
    #[serde(rename="startTime")]
    start_time: Option<u64>,
    #[serde(rename="endTime")]
    end_time: Option<u64>,
    #[serde(rename="recvWindow")]
    recv_window: Option<u64>,
}


#[derive(Deserialize, Debug)]
pub struct GetWithdrawHistoryResp {
    pub id: String,
    pub amount: BigDecimal,
    #[serde(rename="transactionFee")]
    pub transaction_fee:  BigDecimal,
    pub coin: String,
    pub status: u8,
    pub address: String,
    #[serde(rename="txId")]
    pub tx_id: String,
    #[serde(rename="applyTime")]
    pub apply_time: String,
    pub network: String,
    #[serde(rename="transferType")]
    pub transfer_type: u8,
    #[serde(rename="withdrawOrderId")]
    pub withdraw_order_id: String,
    pub info: String,
    #[serde(rename="confirmNo")]
    pub confirm_no: u64,
    #[serde(rename="walletType")]
    pub wallet_type: u8,
    #[serde(rename="txKey")]
    pub tx_key: String,
    #[serde(rename="completeTime")]
    pub complete_time: String,
}


#[derive(Deserialize, Debug)]
pub struct GetWithdrawAddressResp {
    pub address: String,
    #[serde(rename="addressTag")]
    pub address_tag: String,
    pub coin: String,
    pub name: String,
    pub network: String,
    pub origin: String,
    #[serde(rename="originType")]
    pub origin_type: String,
    #[serde(rename="whiteStatus")]
    pub white_status: bool,
}
