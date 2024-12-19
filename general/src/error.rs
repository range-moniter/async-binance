use ed25519_dalek::pkcs8;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("Binance API error: {0}")]
    HttpError(#[from] hyper_util::client::legacy::Error),
    #[error("hyper error: {0}")]
    HyperError(#[from] hyper::Error),
    #[error("application error")]
    ApplicationError(#[from] ApplicationError),
    #[error("websocket streaming client error")]
    WebsocketError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("deserializer error")]
    DeserializeError(#[from] serde_json::Error),
    #[error("binance error: {0}")]
    BinanceError(BinanceError),
    #[error("failed to deserialize api response: {0}")]
    JSONDeserialize(serde_json::Error),
    #[error("failed to sign with ed25519: {0}")]
    SignEd25519Error(#[from] pkcs8::Error),
    #[error("failed to url encoded metadata: {0}")]
    UrlEncodedError(#[from] serde_urlencoded::ser::Error),
    #[error("failed to got response frame bytes")]
    ResponseBodyFrameError(String),
    #[error("parameter error: {0}")]
    ParameterError(String),
}
#[derive(Debug, Error)]
pub struct ApplicationError(pub String);

impl ApplicationError {
    pub const fn new(message: String) -> ApplicationError {
        ApplicationError(message)
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct BinanceError {
    msg: String,
    #[serde(rename = "code", deserialize_with = "deserialization_api_resp_error")]
    error_type: ErrorType,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type={},msg={}", self.error_type, self.msg)
    }
}

#[derive(Debug, Deserialize)]
pub enum ErrorType {
    NotMatch = 0,
    #[serde(rename = "UNKNOWN")]
    UnKnown = -1000,
    #[serde(rename = "DISCONNECTED")]
    Disconnected = -1001,
    #[serde(rename = "UNAUTHORIZED")]
    UnAuthorized = -1002,
    #[serde(rename = "TOO_MANY_REQUESTS")]
    TooManyRequest = -1003,
    #[serde(rename = "UNEXPECTED_RESP")]
    UnExpectedResp = -1006,
    #[serde(rename = "TIMEOUT")]
    Timeout = -1007,
    #[serde(rename = "SERVER_BUSY")]
    ServerBusy = -1008,
    #[serde(rename = "INVALID_MESSAGE")]
    InvalidMessage = -1013,
    #[serde(rename = "UNKNOWN_ORDER_COMPOSITION")]
    UnKnownOrderComposition = -1014,
    #[serde(rename = "TOO_MANY_ORDERS")]
    TooManyOrders = -1015,
    #[serde(rename = "SERVICE_SHUTTING_DOWN")]
    ServiceShuttingDown = -1016,
    #[serde(rename = "UNSUPPORTED_OPERATION")]
    UnSupportedOperation = -1020,
    #[serde(rename = "INVALID_TIMESTAMP")]
    InvalidTimestamp = -1021,
    #[serde(rename = "INVALID_SIGNATURE")]
    InvalidSignature = -1022,
    #[serde(rename = "ILLEGAL_CHARS")]
    IllegalCharset = -1100,
    #[serde(rename = "TOO_MANY_PARAMETERS")]
    TooManyParameters = -1101,
    #[serde(rename = "MANDATORY_PARAM_EMPTY_OR_MALFORMED")]
    MandatoryParamEmptyOrMalformed = -1102,
    #[serde(rename = "UNKNOWN_PARAM")]
    UnKnownParam = -1103,
    #[serde(rename = "UNREAD_PARAMETERS")]
    UnReadParameters = -1104,
    #[serde(rename = "PARAM_EMPTY")]
    ParamEmpty = -1105,
    #[serde(rename = "PARAM_NOT_REQUIRED")]
    ParamNotRequired = -1106,
    #[serde(rename = "PARAM_OVERFLOW")]
    ParamOverflow = -1108,
    #[serde(rename = "BAD_PRECISION")]
    BadPrecision = -1111,
    #[serde(rename = "NO_DEPTH")]
    NoDepth = -1112,
    #[serde(rename = "TIF_NOT_REQUIRED")]
    TIFNotRequired = -1114,
    #[serde(rename = "INVALID_TIF")]
    InvalidTIF = -1115,
    #[serde(rename = "INVALID_ORDER_TYPE")]
    InvalidOrderType = -1116,
    #[serde(rename = "INVALID_SIDE")]
    InvalidSide = -1117,
    #[serde(rename = "EMPTY_NEW_CL_ORD_ID")]
    EmptyNewClOrdId = -1118,
    #[serde(rename = "EMPTY_ORG_CL_ORD_ID")]
    EmptyOr4gClOrdId = -1119,
    #[serde(rename = "BAD_INTERVAL")]
    BadInterval = -1120,
    #[serde(rename = "BAD_SYMBOL")]
    BadSymbol = -1121,
    #[serde(rename = "INVALID_SYMBOLSTATUS")]
    InvalidSymbolStatus = -1122,
    #[serde(rename = "INVALID_LISTEN_KEY")]
    InvalidListenKey = -1125,
    #[serde(rename = "MORE_THAN_XX_HOURS")]
    MoreThanSomeHours = -1127,
    #[serde(rename = "OPTIONAL_PARAMS_BAD_COMBO")]
    OptionalParamsBadCombo = -1128,
    #[serde(rename = "INVALID_PARAMETER")]
    InvalidParameter = -1130,
    #[serde(rename = "BAD_STRATEGY_TYPE")]
    BadStrategyType = -1134,
    #[serde(rename = "INVALID_JSON")]
    InvalidJson = -1135,
    #[serde(rename = "INVALID_TICKER_TYPE")]
    InvalidTickerType = -1139,
    #[serde(rename = "INVALID_CANCEL_RESTRICTIONS")]
    InvalidCancelRestrictions = -1145,
    #[serde(rename = "DUPLICATE_SYMBOLS")]
    DuplicateSymbols = -1151,
    #[serde(rename = "INVALID_SBE_HEADER")]
    InvalidSbeHeaders = -1152,
    #[serde(rename = "UNSUPPORTED_SCHEMA_ID")]
    UnSupportedSchemaId = -1153,
    #[serde(rename = "SBE_DISABLED")]
    SbeDisabled = -1155,
    #[serde(rename = "OCO_ORDER_TYPE_REJECTED")]
    OCOOrderTypeRejected = -1158,
    #[serde(rename = "OCO_ICEBERGQTY_TIMEINFORCE")]
    OCOIcebergQtyTimeInForce = -1160,
    #[serde(rename = "BUY_OCO_LIMIT_MUST_BE_BELOW")]
    BuyOCOLimitMustBeBelow = -1165,
    #[serde(rename = "SELL_OCO_LIMIT_MUST_BE_ABOVE")]
    SellOCOLimitMustBeAbove = -1166,
    #[serde(rename = "BOTH_OCO_ORDERS_CANNOT_BE_CONTINGENT")]
    BothOCOOrdersCannotBeContingent = -1167,
    #[serde(rename = "BOTH_OCO_ORDERS_CANNOT_BE_LIMIT")]
    BothOCOOrdersCannotBeLimit = -1168,
    #[serde(rename = "INVALID_TAG_NUMBER")]
    InvalidTagNumber = -1169,
    #[serde(rename = "TAG_NOT_DEFINED_IN_MESSAGE")]
    TagNotDefinedInMessage = -1170,
    #[serde(rename = "TAG_APPEARS_MORE_THAN_ONCE")]
    TagAppearsMoreThanOnce = -1171,
    #[serde(rename = "TAG_OUT_OF_ORDER")]
    TagOutOfOrder = -1172,
    #[serde(rename = "GROUP_FIELDS_OUT_OF_ORDER")]
    GroupFieldsOutOfOrder = -1173,
    #[serde(rename = "INVALID_COMPONENT")]
    InvalidComponent = -1174,
    #[serde(rename = "RESET_SEQ_NUM_SUPPORT")]
    ResetSeqNumSupport = -1175,
    #[serde(rename = "ALREADY_LOGGED_IN")]
    AlreadyLogged = -1176,
    #[serde(rename = "GARBLED_MESSAGE")]
    GarbledMessage = -1177,
    #[serde(rename = "BAD_SENDER_COMPID")]
    BadSenderCompId = -1178,
    #[serde(rename = "BAD_SEQ_NUM")]
    BadSeqNum = -1179,
    #[serde(rename = "EXPECTED_LOGON")]
    ExpectedLogon = -1180,
    #[serde(rename = "TOO_MANY_MESSAGES")]
    TooManyMessages = -1181,
    #[serde(rename = "PARAMS_BAD_COMBO")]
    ParamsBadCombo = -1182,
    #[serde(rename = "NOT_ALLOWED_IN_DROP_COPY_SESSIONS")]
    NotAllowedInDropCopySessions = -1183,
    #[serde(rename = "DROP_COPY_SESSION_NOT_ALLOWED")]
    DropCopySessionNotAllowed = -1184,
    #[serde(rename = "DROP_COPY_SESSION_REQUIRED")]
    DropCopySessionRequired = -1185,
    #[serde(rename = "NEW_ORDER_REJECTED")]
    NewOrderRejected = -2010,
    #[serde(rename = "CANCEL_REJECTED")]
    CancelRejected = -2011,
    #[serde(rename = "NO_SUCH_ORDER")]
    NoSuchOrder = -2013,
    #[serde(rename = "BAD_API_KEY_FMT")]
    BadApiKeyFmt = -2014,
    #[serde(rename = "REJECTED_MBX_KEY")]
    RejectedMbxKey = -2015,
    #[serde(rename = "NO_TRADING_WINDOW")]
    NoTradingWindow = -2016,
    #[serde(rename = "ORDER_ARCHIVED")]
    OrderArchived = -2026,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<i32> for ErrorType {
    fn from(value: i32) -> Self {
        match value {
            -2026 => ErrorType::OrderArchived,
            -2016 => ErrorType::NoTradingWindow,
            -2015 => ErrorType::RejectedMbxKey,
            -2014 => ErrorType::BadApiKeyFmt,
            -2013 => ErrorType::NoSuchOrder,
            -2011 => ErrorType::CancelRejected,
            -2010 => ErrorType::NewOrderRejected,
            -1185 => ErrorType::DropCopySessionRequired,
            -1184 => ErrorType::DropCopySessionNotAllowed,
            -1183 => ErrorType::NotAllowedInDropCopySessions,
            -1182 => ErrorType::ParamsBadCombo,
            -1181 => ErrorType::TooManyMessages,
            -1180 => ErrorType::ExpectedLogon,
            -1179 => ErrorType::BadSeqNum,
            -1178 => ErrorType::BadSenderCompId,
            -1177 => ErrorType::GarbledMessage,
            -1176 => ErrorType::AlreadyLogged,
            -1175 => ErrorType::ResetSeqNumSupport,
            -1174 => ErrorType::InvalidComponent,
            -1173 => ErrorType::GroupFieldsOutOfOrder,
            -1172 => ErrorType::TagOutOfOrder,
            -1171 => ErrorType::TagAppearsMoreThanOnce,
            -1170 => ErrorType::TagNotDefinedInMessage,
            -1169 => ErrorType::InvalidTagNumber,
            -1168 => ErrorType::BothOCOOrdersCannotBeLimit,
            -1167 => ErrorType::BothOCOOrdersCannotBeContingent,
            -1166 => ErrorType::SellOCOLimitMustBeAbove,
            -1165 => ErrorType::BuyOCOLimitMustBeBelow,
            -1160 => ErrorType::OCOIcebergQtyTimeInForce,
            -1158 => ErrorType::OCOOrderTypeRejected,
            -1155 => ErrorType::SbeDisabled,
            -1153 => ErrorType::UnSupportedSchemaId,
            -1152 => ErrorType::InvalidSbeHeaders,
            -1151 => ErrorType::DuplicateSymbols,
            -1145 => ErrorType::InvalidCancelRestrictions,
            -1139 => ErrorType::InvalidTickerType,
            -1135 => ErrorType::InvalidJson,
            -1134 => ErrorType::BadStrategyType,
            -1130 => ErrorType::InvalidParameter,
            -1128 => ErrorType::OptionalParamsBadCombo,
            -1127 => ErrorType::MoreThanSomeHours,
            -1125 => ErrorType::InvalidListenKey,
            -1122 => ErrorType::InvalidSymbolStatus,
            -1121 => ErrorType::BadSymbol,
            -1120 => ErrorType::BadInterval,
            -1119 => ErrorType::EmptyOr4gClOrdId,
            -1118 => ErrorType::EmptyNewClOrdId,
            -1117 => ErrorType::InvalidSide,
            -1116 => ErrorType::InvalidOrderType,
            -1115 => ErrorType::InvalidTIF,
            -1114 => ErrorType::TIFNotRequired,
            -1112 => ErrorType::NoDepth,
            -1111 => ErrorType::BadPrecision,
            -1108 => ErrorType::ParamOverflow,
            -1106 => ErrorType::ParamNotRequired,
            -1105 => ErrorType::ParamEmpty,
            -1104 => ErrorType::UnReadParameters,
            -1103 => ErrorType::UnKnownParam,
            -1102 => ErrorType::MandatoryParamEmptyOrMalformed,
            -1101 => ErrorType::TooManyParameters,
            -1100 => ErrorType::IllegalCharset,
            -1022 => ErrorType::InvalidSignature,
            -1021 => ErrorType::InvalidTimestamp,
            -1020 => ErrorType::UnSupportedOperation,
            -1016 => ErrorType::ServiceShuttingDown,
            -1015 => ErrorType::TooManyOrders,
            -1014 => ErrorType::UnKnownOrderComposition,
            -1013 => ErrorType::InvalidMessage,
            -1008 => ErrorType::ServerBusy,
            -1007 => ErrorType::Timeout,
            -1006 => ErrorType::UnExpectedResp,
            -1003 => ErrorType::TooManyRequest,
            -1002 => ErrorType::UnAuthorized,
            -1001 => ErrorType::Disconnected,
            -1000 => ErrorType::UnKnown,
            _ => ErrorType::NotMatch,
        }
    }
}

fn deserialization_api_resp_error<'de, D>(deserializer: D) -> Result<ErrorType, D::Error>
where
    D: Deserializer<'de>,
{
    let code = i32::deserialize(deserializer)?;
    Ok(ErrorType::from(code))
}
pub fn map_deserialization_error(e: serde_json::Error, bytes: &[u8]) -> SdkError {
    log::debug!(
        "Failed to deserialization data. data={}, error={}",
        String::from_utf8_lossy(bytes),
        e
    );
    SdkError::JSONDeserialize(e)
}