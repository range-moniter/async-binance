use crate::types::order::general::cancel::{
    CancelOrderReq, CancelOrderResp, CancelSingleTypeOrderReq,
};
use crate::types::order::general::create::{CreateOrderReq, CreateOrderResp};
use crate::types::order::general::query::{QueryAllOrderReq, QueryOrderReq, QueryOrderResp};
use crate::types::order::lists::cancel::{CancelOrderListReq, CancelOrderListResp};
use crate::types::order::lists::oco_create::{CreateOcoOrderReq, CreateOcoOrderResp};
use crate::types::order::lists::oto_create::{CreateOtoOrderReq, CreateOtoOrderResp};
use crate::types::order::lists::otoco_create::{CreateOtoCoOrderReq, CreateOtoCoOrderResp};
use crate::types::order::lists::query::{QueryOrderListReq, QueryOrderListResp};
use crate::types::order::lists::query_open::{QueryOpenOrderReq, QueryOpenOrderResp};
use crate::types::order::sor::create::{CreateSorOrderReq, CreateSorOrderResp};
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use client::rest::layer::authorization::types::{AuthType, Certificate};
use general::result::BinanceResult;

pub struct SpotOrderClient<T> {
    client: T,
    domain: String,
}

impl<T> SpotOrderClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> Self {
        SpotOrderClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }

    async fn create_order(
        &self,
        req: CreateOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CreateOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/order",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::UserData, 1, certificate, uid),
            )
            .await
    }
    async fn get_orders(
        &self,
        req: QueryOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<QueryOrderResp> {
        self.client
            .get(
                req,
                "/api/v3/order",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserData, 4, certificate),
            )
            .await
    }

    async fn get_all_orders(
        &self,
        req: QueryAllOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<QueryOrderResp>> {
        self.client
            .get(
                req,
                "/api/v3/allOrders",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserData, 20, certificate),
            )
            .await
    }

    async fn cancel_order(
        &self,
        req: CancelOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CancelOrderResp> {
        self.client
            .delete(
                req,
                "/api/v3/order",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::UserData, 1, certificate, uid),
            )
            .await
    }

    async fn cancel_single_symbol_all_order(
        &self,
        req: CancelSingleTypeOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<Vec<CancelOrderResp>> {
        self.client
            .delete(
                req,
                "/api/v3/openOrders",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::UserData, 1, certificate, uid),
            )
            .await
    }

    async fn create_oco_order(
        &self,
        req: CreateOcoOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CreateOcoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/oco",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::Trade, 1, certificate, uid),
            )
            .await
    }

    async fn create_oto_order(
        &self,
        req: CreateOtoOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CreateOtoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/oto",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::Trade, 1, certificate, uid),
            )
            .await
    }

    async fn create_oto_co_order(
        &self,
        req: CreateOtoCoOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CreateOtoCoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/otoco",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::Trade, 1, certificate, uid),
            )
            .await
    }

    async fn cancel_order_list(
        &self,
        req: CancelOrderListReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CancelOrderListResp> {
        self.client
            .delete(
                req,
                "/api/v3/orderList",
                self.domain.as_str(),
                RequestExtension::auth_order_api(AuthType::Trade, 1, certificate, uid),
            )
            .await
    }

    async fn get_order_list(
        &self,
        req: QueryOrderListReq,
        certificate: Certificate,
    ) -> BinanceResult<QueryOrderListResp> {
        self.client
            .get(
                req,
                "/api/v3/orderList",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserData, 4, certificate),
            )
            .await
    }

    async fn get_all_order_list(
        &self,
        req: QueryOrderListReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<QueryOrderListResp>> {
        self.client
            .get_multiple(
                req,
                "/api/v3/allOrderList",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserData, 20, certificate),
            )
            .await
    }

    async fn get_open_order_list(
        &self,
        req: QueryOpenOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<QueryOpenOrderResp>> {
        self.client
            .get_multiple(
                req,
                "/api/v3/openOrderList",
                self.domain.as_str(),
                RequestExtension::auth_api(AuthType::UserData, 6, certificate),
            )
            .await
    }

    async fn create_new_sor_order(
        &self,
        req: CreateSorOrderReq,
        certificate: Certificate,
        uid: u64,
    ) -> BinanceResult<CreateSorOrderResp> {
        self.client.post(
            req,
            "/api/v3/sor/order",
            self.domain.as_str(),
            RequestExtension::auth_order_api(AuthType::Trade, 1, certificate, uid),
        ).await
    }
}