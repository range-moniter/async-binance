use crate::rest::layer::rate::types::RateType;
use crate::rest::layer::rate::types::window::{WeightWindow, WindowUnit};

#[derive(Debug, Clone)]
pub struct IpWeightHandle {
    api_window: WeightWindow,
    sapi_window: WeightWindow,
}

impl IpWeightHandle {
    pub fn new_default() -> Self {
        IpWeightHandle {
            api_window: WeightWindow::new(6000, 1, WindowUnit::Minute),
            sapi_window: WeightWindow::new(12000, 1, WindowUnit::Minute),
        }
    }

    pub fn available(&mut self, weight: u32, rate_type: Option<RateType>) -> bool {
        if rate_type.is_none() {
            return true;
        }
        let rate_type = rate_type.unwrap();
        if rate_type.is_sapi() {
            self.sapi_window.check_weight(weight)
        } else {
            self.api_window.check_weight(weight)
        }
    }
}