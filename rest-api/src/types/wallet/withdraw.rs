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

#[derive(Serialize, Debug, Default)]
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


pub struct GetWithdrawHistoryReqBuilder {
    req: GetWithdrawHistoryReq
}

impl GetWithdrawHistoryReqBuilder {
    pub fn new_builder() -> Self {
        Self { req: Default::default() }
    }

    pub fn set_coin(mut self, coin: &str) -> Self {
        self.req.coin = Some(coin.to_owned());
        self
    }
    pub fn set_withdraw_order_id(mut self, withdraw_order_id: &str) -> Self {
        self.req.withdraw_order_id = Some(withdraw_order_id.to_owned());
        self
    }
    pub fn set_status(mut self, status: u8) -> Self {
        self.req.status = Some(status);
        self
    }
    pub fn set_offset(mut self, offset: u32) -> Self {
        self.req.offset = Some(offset);
        self
    }
    pub fn set_limit(mut self, limit: u32) -> Self {
        self.req.limit = Some(limit);
        self
    }
    pub fn set_id_list(mut self, id_list: &str) -> Self {
        self.req.id_list = Some(id_list.to_owned());
        self
    }
    pub fn set_start(mut self, start_time: u64) -> Self {
        self.req.start_time = Some(start_time);
        self
    }
    pub fn set_end_time(mut self, end_time: u64) -> Self {
        self.req.end_time = Some(end_time);
        self
    }
    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.req.recv_window = Some(recv_window);
        self
    }
    pub fn build(mut self) -> GetWithdrawHistoryReq {

        // network may not be in the response for old withdraw.
        // Please notice the default startTime and endTime to make sure that time interval is within 0-90 days.
        // If both startTime and endTimeare sent, time between startTimeand endTimemust be less than 90 days.
        // If withdrawOrderId is sent, time between startTime and endTime must be less than 7 days.
        // If withdrawOrderId is sent, startTime and endTime are not sent, will return last 7 days records by default.
        // Maximum support idList number is 45.

        // if !self.req.withdraw_order_id.is_none() {
        //
        // }
        self.req
    }
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
    pub network: Option<String>,
    #[serde(rename="transferType")]
    pub transfer_type: u8,
    #[serde(rename="withdrawOrderId")]
    pub withdraw_order_id: Option<String>,
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
