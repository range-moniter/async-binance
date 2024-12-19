use serde::Deserialize;

pub mod oco_create;
pub mod oto_create;
pub mod otoco_create;
pub mod cancel;
pub mod query;
pub mod query_all;
pub mod query_open;

#[derive(Debug, Deserialize)]
pub struct OrderId {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
}