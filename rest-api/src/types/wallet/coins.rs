use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetWalletCoinsReq {
    #[serde(rename = "recvWindow")]
    recv_window: Option<u64>,
}
#[derive(Deserialize, Debug)]
pub struct GetWalletCoinsResp {
    pub coin: String,
    #[serde(rename = "depositAllEnable")]
    pub deposit_all_enable: bool,
    pub free: BigDecimal,
    pub freeze: BigDecimal,
    #[serde(rename = "ipoable")]
    pub ipo_able: BigDecimal,
    #[serde(rename = "ipoing")]
    pub ipo_ing: BigDecimal,
    #[serde(rename = "isLegalMoney")]
    pub is_legal_money: bool,
    pub locked: BigDecimal,
    pub name: String,
    #[serde(rename = "networkList")]
    pub networks: Vec<CoinNetworkResp>,
    pub storage: BigDecimal,
    pub trading: bool,
    #[serde(rename = "withdrawAllEnable")]
    pub with_draw_all_enable: bool,
    #[serde(rename = "withdrawing")]
    pub with_drawing: BigDecimal
}

#[derive(Deserialize, Debug)]
pub struct CoinNetworkResp {
    #[serde(rename = "addressRegex")]
    pub address_regex: String,
    pub coin: String,
    #[serde(rename = "depositDesc")]
    pub deposit_desc: String,
    #[serde(rename = "depositEnable")]
    pub deposit_enable: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "memoRegex")]
    pub memo_regex: String,
    #[serde(rename = "minConfirm")]
    pub min_confirm: u64,
    pub name: String,
    pub network: String,
    #[serde(rename = "specialTips")]
    pub special_tips: String,
    #[serde(rename = "unLockConfirm")]
    pub unlock_confirm: u64,
    #[serde(rename = "withdrawDesc")]
    pub withdraw_desc: String,
    #[serde(rename = "withdrawEnable")]
    pub withdraw_enable: bool,
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: BigDecimal,
    #[serde(rename = "withdrawIntegerMultiple")]
    pub withdraw_integer_multiple: BigDecimal,
    #[serde(rename = "withdrawMax")]
    pub withdraw_max: BigDecimal,
    #[serde(rename = "withdrawMin")]
    pub withdraw_min: BigDecimal,
    #[serde(rename = "withdrawInternalMin")]
    pub withdraw_internal_min: BigDecimal,
    #[serde(rename = "sameAddress")]
    pub same_address: bool,
    #[serde(rename = "estimatedArrivalTime")]
    pub estimated_arrival_time: u64,
    pub busy: bool,
    #[serde(rename = "contractAddressUrl")]
    pub contract_address_url: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String
}