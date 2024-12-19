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
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use client::rest::layer::authorization::types::{AuthType, Certificate};
use general::result::BinanceResult;

pub struct OrderClient<T> {
    client: T,
    domain: String,
}

impl<T> OrderClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> Self {
        OrderClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }

    async fn create_order(
        &self,
        req: CreateOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<CreateOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/order",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 1, certificate),
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
                RequestExtension::auth(AuthType::UserData, 4, certificate),
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
                RequestExtension::auth(AuthType::UserData, 20, certificate),
            )
            .await
    }

    async fn cancel_order(
        &self,
        req: CancelOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<CancelOrderResp> {
        self.client
            .delete(
                req,
                "/api/v3/order",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 1, certificate),
            )
            .await
    }

    async fn cancel_single_symbol_all_order(
        &self,
        req: CancelSingleTypeOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<CancelOrderResp>> {
        self.client
            .delete(
                req,
                "/api/v3/openOrders",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 1, certificate),
            )
            .await
    }

    async fn create_oco_order(
        &self,
        req: CreateOcoOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<CreateOcoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/oco",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::Trade, 1, certificate),
            )
            .await
    }

    async fn create_oto_order(
        &self,
        req: CreateOtoOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<CreateOtoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/oto",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::Trade, 1, certificate),
            )
            .await
    }

    async fn create_oto_co_order(
        &self,
        req: CreateOtoCoOrderReq,
        certificate: Certificate,
    ) -> BinanceResult<CreateOtoCoOrderResp> {
        self.client
            .post(
                req,
                "/api/v3/orderList/otoco",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::Trade, 1, certificate),
            )
            .await
    }

    async fn cancel_order_list(
        &self,
        req: CancelOrderListReq,
        certificate: Certificate,
    ) -> BinanceResult<CancelOrderListResp> {
        self.client
            .delete(
                req,
                "/api/v3/orderList",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::Trade, 1, certificate),
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
                RequestExtension::auth(AuthType::UserData, 4, certificate),
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
                RequestExtension::auth(AuthType::UserData, 20, certificate),
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
                RequestExtension::auth(AuthType::UserData, 6, certificate),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::order::general::create::CreateOrderReqBuilder;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use client::rest::config::Config;
    use client::rest::rest_client::BinanceRestClient;
    use general::enums::general::TimeInForce;
    use general::enums::order::{OrderSide, OrderType};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref CLIENT: OrderClient<BinanceRestClient> =
            OrderClient::new(BinanceRestClient::build_client(Config::new_default()));
        static ref CERTIFICATE: Certificate = Certificate::new("", "");
    }
    #[tokio::test]
    async fn test_create_order() {
        let req = CreateOrderReqBuilder::new_builder()
            .symbol("DOGEUSDT")
            .side(OrderSide::BUY)
            .order_type(OrderType::LIMIT)
            .time_in_force(TimeInForce::GTC)
            .quantity(BigDecimal::from_f32(10.0).unwrap().with_scale(2))
            .price(BigDecimal::from_f32(0.4).unwrap().with_scale(4))
            .build();
        match req {
            Ok(req) => {
                let resp = CLIENT.create_order(req, CERTIFICATE.clone()).await;
                println!("{:?}", resp)
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
    #[tokio::test]
    async fn test_get_orders() {
        let req = QueryOrderReq::new_with_order_id("DOGEUSDT", 7483831693);
        let resp = CLIENT.get_orders(req, CERTIFICATE.clone()).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_cancel_order() {
        let req = CancelOrderReq::new_with_order_id("DOGEUSDT", 7483831693);
        let resp = CLIENT.cancel_order(req, CERTIFICATE.clone()).await;
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_all_orders() {
        let req = QueryAllOrderReq::new("DOGEUSDT");
        let resp = CLIENT.get_all_orders(req, CERTIFICATE.clone()).await;
        println!("{:?}", resp);
    }
}
