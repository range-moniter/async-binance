use crate::userdata::types::account::AccountPositionEvent;
use crate::userdata::types::balance::BalanceEvent;
use crate::userdata::types::listen_key::{ExternalLockedEvent, ListenKeyExpireEvent};
use crate::userdata::types::order::OrderEvent;
use crate::userdata::types::order_list::OrderListEvent;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "e")]
pub enum UserDataEventPayload {
    #[serde(rename = "outboundAccountPosition")]
    OutboundAccountPosition(AccountPositionEvent),
    #[serde(rename = "balanceUpdate")]
    BalanceUpdate(BalanceEvent),
    #[serde(rename = "executionReport")]
    Order(OrderEvent),
    #[serde(rename = "listStatus")]
    OrderList(OrderListEvent),
    #[serde(rename = "listenKeyExpired")]
    ListenKeyExpired(ListenKeyExpireEvent),
    #[serde(rename = "externalLockUpdate")]
    ExternalLockUpdate(ExternalLockedEvent),
    #[serde(rename = "eventStreamTerminated")]
    EventStreamTerminated,
}