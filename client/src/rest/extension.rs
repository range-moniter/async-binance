use crate::rest::layer::authorization::types::{AuthType, Certificate};
use hyper::http::Extensions;
use crate::rest::layer::rate::types::RateType;

#[derive(Debug, Clone)]
pub enum RequestExtension {
    OrderRate(RateType),
    UidRate(RateType),
    IpRate(RateType),
    Weight(u32),
    Auth(AuthType),
    WindowSize(u16),
    Cert(Certificate),
    Body(String),
    Param(String),
}

impl RequestExtension {
    pub fn none_auth_api(weight: u32) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(AuthType::None),
            RequestExtension::Weight(weight),
            RequestExtension::IpRate(RateType::IpWeightRate(false)),
        ]
    }

    pub fn none_auth_sapi(weight: u32) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(AuthType::None),
            RequestExtension::Weight(weight),
            RequestExtension::IpRate(RateType::IpWeightRate(true)),
        ]
    }

    pub fn auth_api(
        auth_type: AuthType,
        weight: u32,
        certificate: Certificate,
    ) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(auth_type),
            RequestExtension::Weight(weight),
            RequestExtension::Cert(certificate),
            RequestExtension::IpRate(RateType::IpWeightRate(false)),
        ]
    }

    pub fn auth_sapi(auth_type: AuthType, weight: u32, certificate: Certificate) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(auth_type),
            RequestExtension::Weight(weight),
            RequestExtension::Cert(certificate),
            RequestExtension::IpRate(RateType::IpWeightRate(true)),
        ]
    }

    pub fn auth_uid_sapi(auth_type: AuthType, weight: u32, certificate: Certificate, uid: u64) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(auth_type),
            RequestExtension::Weight(weight),
            RequestExtension::Cert(certificate),
            RequestExtension::UidRate(RateType::UidRate(true, uid))
        ]
    }

    pub fn auth_order_api(auth_type: AuthType, weight: u32, certificate: Certificate, uid: u64) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(auth_type),
            RequestExtension::Weight(weight),
            RequestExtension::Cert(certificate),
            RequestExtension::IpRate(RateType::IpWeightRate(false)),
            RequestExtension::OrderRate(RateType::OrderRate(uid))
        ]
    }

    pub fn explain_auth_type(
        extension: &Extensions,
    ) -> (Option<AuthType>, Option<u16>, Option<Certificate>) {
        let extension = extension.get::<Vec<RequestExtension>>().unwrap();
        let auth_type = extension.iter().find_map(|ext| match ext {
            RequestExtension::Auth(auth_type) => Some(*auth_type),
            _ => None,
        });
        let window_size = extension.iter().find_map(|ext| match ext {
            RequestExtension::WindowSize(size) => Some(*size),
            _ => None,
        });
        let certificate = extension.iter().find_map(|ext| match ext {
            RequestExtension::Cert(certificate) => Some(certificate.clone()),
            _ => None,
        });
        (auth_type, window_size, certificate)
    }

    pub fn explain_request_weight(extension: &Extensions) -> Option<u32> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::Weight(weight) => Some(*weight),
            _ => None,
        })
    }

    pub fn explain_request_order_rate(extension: &Extensions) -> Option<RateType> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::OrderRate(rate) => Some(rate.clone()),
            _ => None,
        })
    }

    pub fn explain_request_uid_rate(extension: &Extensions) -> Option<RateType> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::UidRate(rate) => Some(rate.clone()),
            _ => None,
        })
    }

    pub fn explain_request_ip_rate(extension: &Extensions) -> Option<RateType> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::IpRate(rate) => Some(rate.clone()),
            _ => None,
        })
    }

    pub fn explain_request_params(extension: &Extensions) -> Option<String> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::Param(param) => Some(param.clone()),
            _ => None,
        })
    }

    pub fn explain_request_body(extension: &Extensions) -> Option<String> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::Body(body) => Some(body.clone()),
            _ => None,
        })
    }
}
