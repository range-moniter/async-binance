use crate::types::wallet::coins::{GetWalletCoinsReq, GetWalletCoinsResp};
use crate::types::wallet::withdraw::{
    CreateWithdrawReq, CreateWithdrawResp, GetWithdrawHistoryReq, GetWithdrawHistoryResp,
};
use client::rest::client::{BinanceClient, BinanceClientAction};
use client::rest::extension::RequestExtension;
use client::rest::layer::authorization::types::{AuthType, Certificate};
use general::result::BinanceResult;

pub struct WalletClient<T> {
    client: T,
    domain: String,
}

impl<T> WalletClient<T>
where
    T: BinanceClient + BinanceClientAction,
{
    pub fn new(client: T) -> Self {
        WalletClient {
            client,
            domain: "api.binance.com".to_string(),
        }
    }
    // link: https://developers.binance.com/docs/wallet/capital
    async fn get_all_wallet_coins(
        &self,
        req: GetWalletCoinsReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<GetWalletCoinsResp>> {
        self.client
            .get_multiple(
                req,
                "/sapi/v1/capital/config/getall",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 10, certificate),
            )
            .await
    }

    // link: https://developers.binance.com/docs/wallet/capital/withdraw
    async fn submit_withdraw(
        &self,
        req: CreateWithdrawReq,
        certificate: Certificate,
    ) -> BinanceResult<CreateWithdrawResp> {
        self.client
            .post(
                req,
                "/sapi/v1/capital/withdraw/apply",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 600, certificate),
            )
            .await
    }

    // link: https://developers.binance.com/docs/wallet/capital/withdraw-history
    async fn get_withdraw_history(
        &self,
        req: GetWithdrawHistoryReq,
        certificate: Certificate,
    ) -> BinanceResult<Vec<GetWithdrawHistoryResp>> {
        self.client
            .get_multiple(
                req,
                "/sapi/v1/capital/withdraw/history",
                self.domain.as_str(),
                RequestExtension::auth(AuthType::UserData, 1800, certificate),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::rest::config::Config;
    use client::rest::rest_client::BinanceRestClient;
    use env_logger::Builder;
    use lazy_static::lazy_static;
    use crate::types::wallet::withdraw::GetWithdrawHistoryReqBuilder;

    lazy_static! {
        static ref CLIENT: WalletClient<BinanceRestClient> =
            WalletClient::new(BinanceRestClient::build_client(Config::new_default()));
        static ref CERTIFICATE: Certificate = Certificate::new(
            "",
            ""
        );
    }
    #[tokio::test]
    async fn test_get_wallet_coins() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();
        let resp = CLIENT
            .get_all_wallet_coins(GetWalletCoinsReq::new(None), CERTIFICATE.clone())
            .await
            .unwrap();
        println!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_withdraw_history() {
        Builder::from_default_env()
            .filter(None, log::LevelFilter::Debug)
            .init();
        let req = GetWithdrawHistoryReqBuilder::new_builder().build();
        let resp = CLIENT.get_withdraw_history(req, CERTIFICATE.clone()).await.unwrap();
        println!("{:?}", resp);
    }
}
