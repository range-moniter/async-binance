use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct GetWalletCoinsReq {
    #[serde(rename = "recvWindow")]
    recv_window: Option<u64>,
}

impl GetWalletCoinsReq {
    pub fn new(recv_window: Option<u64>) -> Self {
        Self { recv_window }
    }
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
    pub special_tips: Option<String>,
    #[serde(rename = "unLockConfirm")]
    pub unlock_confirm: u64,
    #[serde(rename = "withdrawDesc")]
    pub withdraw_desc: Option<String>,
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
    pub withdraw_internal_min: Option<BigDecimal>,
    #[serde(rename = "sameAddress")]
    pub same_address: bool,
    #[serde(rename = "estimatedArrivalTime")]
    pub estimated_arrival_time: u64,
    pub busy: bool,
    #[serde(rename = "contractAddressUrl")]
    pub contract_address_url: Option<String>,
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>
}
//"addressRegex": "^(bnb1)[0-9a-z]{38}$",
// 	   			"coin": "BTC",
// 	   			"depositDesc": "Wallet Maintenance, Deposit Suspended", // shown only when "depositEnable" is false.
// 	   			"depositEnable": false,
// 	   			"isDefault": false,
// 	   			"memoRegex": "^[0-9A-Za-z\\-_]{1,120}$",
// 	   			"minConfirm": 1,  // min number for balance confirmation
// 	   			"name": "BEP2",
// 	   			"network": "BNB",
// 	   			"specialTips": "Both a MEMO and an Address are required to successfully deposit your BEP2-BTCB tokens to Binance.",
// 	   			"unLockConfirm": 0,  // confirmation number for balance unlock
// 	   			"withdrawDesc": "Wallet Maintenance, Withdrawal Suspended", // shown only when "withdrawEnable" is false.
// 	   			"withdrawEnable": false,
// 	   			"withdrawFee": "0.00000220",
// 	   			"withdrawIntegerMultiple": "0.00000001",
// 	   			"withdrawMax": "9999999999.99999999",
// 	   			"withdrawMin": "0.00000440",
// 	   			"withdrawInternalMin": "0.00000440",  // Minimum internal transfer amount
// 	   			"sameAddress": true,  // If the coin needs to provide memo to withdraw
// 	   			"estimatedArrivalTime": 25,
// 	   			"busy": false,
// 	   			"contractAddressUrl": "https://bscscan.com/token/",
// 	   			"contractAddress": "0x7130d2a12b9bcbfae4f2634d864a1ee1ce3ead9c"

