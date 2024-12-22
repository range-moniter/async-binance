use dashmap::DashMap;
use crate::rest::layer::rate::types::RateType;
use crate::rest::layer::rate::types::window::{WeightWindow, WindowUnit};

#[derive(Debug, Clone)]
pub struct UidWeightHandle {
    sapi_uid_weight: DashMap<u64, WeightWindow>,
    sapi_basic_weight_window: WeightWindow,
    api_uid_weight: DashMap<u64, WeightWindow>,
    api_basic_weight_window: WeightWindow,
}

impl UidWeightHandle {
    pub fn new_with_default() -> UidWeightHandle {
        UidWeightHandle {
            sapi_uid_weight: DashMap::new(),
            sapi_basic_weight_window: WeightWindow::new(180000, 1, WindowUnit::Minute),
            api_uid_weight: DashMap::new(),
            api_basic_weight_window: WeightWindow::new(180000, 1, WindowUnit::Minute),
        }
    }

    fn init_uid_rate(&mut self, uid: u64) {
        if self.sapi_uid_weight.contains_key(&uid) && self.api_uid_weight.contains_key(&uid) {
            return;
        }
        self.api_uid_weight.entry(uid).or_insert(self.api_basic_weight_window.clone());
        self.sapi_uid_weight.entry(uid).or_insert(self.sapi_basic_weight_window.clone());
    }

    fn sapi_check(&mut self, weight: u32, uid: u64) -> bool {
        self.sapi_uid_weight.get_mut(&uid).unwrap().check_weight(weight)
    }

    fn api_check(&mut self, weight: u32, uid: u64) -> bool {
        self.api_uid_weight.get_mut(&uid).unwrap().check_weight(weight)
    }

    pub fn available(&mut self, weight: u32, rate_type: Option<RateType>) -> bool {
        if rate_type.is_none() {
            return true;
        }
        let rate_type = rate_type.unwrap();
        let uid = rate_type.get_uid().unwrap();
        self.init_uid_rate(uid);
        if rate_type.is_sapi() {
            self.sapi_check(weight, uid)
        } else {
            self.api_check(weight, uid)
        }
    }
}