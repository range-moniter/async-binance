use crate::rest::layer::authorization::types::{AuthType, Certificate};
use hyper::http::Extensions;

#[derive(Debug, Clone)]
pub enum RequestExtension {
    Weight(u16),
    Auth(AuthType),
    WindowSize(u16),
    Cert(Certificate),
    Body(String),
    Param(String),
}

impl RequestExtension {
    pub fn none_auth(weight: u16) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(AuthType::None),
            RequestExtension::Weight(weight),
        ]
    }

    pub fn auth(
        auth_type: AuthType,
        weight: u16,
        certificate: Certificate,
    ) -> Vec<RequestExtension> {
        vec![
            RequestExtension::Auth(auth_type),
            RequestExtension::Weight(weight),
            RequestExtension::Cert(certificate),
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

    pub fn explain_request_weight(extension: &Extensions) -> Option<u16> {
        let extension = extension.get::<Vec<RequestExtension>>()?;
        extension.iter().find_map(|ext| match ext {
            RequestExtension::Weight(weight) => Some(*weight),
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
