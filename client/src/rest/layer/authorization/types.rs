
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AuthType {
    None,
    Trade,
    UserData,
    UserStream,
    MarketData,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    api_key: String,
    secret_key: String,
}

impl Certificate {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Certificate {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }
}