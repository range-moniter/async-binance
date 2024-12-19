use serde::{Deserialize, Serialize};
use general::enums::permission::Permission;
use general::enums::rate_limiter::RateLimiter;
use general::enums::symbol_status::SymbolStatus;
use crate::types::market::symbol_info::SymbolInfo;


#[derive(Serialize)]
pub struct ExchangeReq {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
    permissions: Option<Vec<Permission>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showPermissionSets")]
    show_permission_sets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolStatus")]
    symbol_status: Option<SymbolStatus>
}


impl ExchangeReq {
    pub fn new() -> Self{
        ExchangeReq {
            symbol: None,
            symbols: None,
            permissions: None,
            show_permission_sets: None,
            symbol_status: None
        }
    }

    pub fn set_symbol(&mut self, symbol: &str) {
        self.symbol = Some(symbol.to_string());
    }
    pub fn set_symbols(&mut self, symbols: Vec<String>) {
        self.symbols = Some(symbols);
    }
    pub fn set_permissions(&mut self, permissions: Vec<Permission>) {
        self.permissions = Some(permissions);
    }
    pub fn set_show_permission_sets(&mut self, show_permission_sets: bool) {
        self.show_permission_sets = Some(show_permission_sets);
    }
    pub fn set_symbol_status(&mut self, symbol_status: SymbolStatus) {
        self.symbol_status = Some(symbol_status);
    }
}

#[derive(Deserialize, Debug)]
pub struct ExchangeResp {
    pub timezone: String,
    #[serde(rename = "serverTime")]
    pub server_time: u64,
    #[serde(rename = "rateLimits")]
    pub rate_limits: Vec<RateLimiter>,
    pub symbols: Vec<SymbolInfo>,
    #[serde(default="Vec::new", rename="sors")]
    pub sor_s: Vec<SorS>
}

#[derive(Deserialize, Debug)]
pub struct SorS {
    #[serde(rename = "baseAsset")]
    base_asset: String,
    #[serde(rename = "symbols")]
    symbols: Vec<String>
}