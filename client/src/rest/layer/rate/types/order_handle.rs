use dashmap::DashMap;
use crate::rest::layer::rate::types::RateType;
use crate::rest::layer::rate::types::window::{WeightWindow, WindowUnit};

#[derive(Debug, Clone)]
pub struct OrderHandle {
    second_level_order_weight: DashMap<u64, WeightWindow>,
    second_level_order_basic_weight_window: WeightWindow,
    day_level_order_weight: DashMap<u64, WeightWindow>,
    day_level_order_basic_weight_window: WeightWindow,
    minute_level_order_weight: DashMap<u64, WeightWindow>,
    minute_level_order_basic_weight_window: WeightWindow,
}

impl OrderHandle {
    pub fn new_with_default() -> OrderHandle {
        OrderHandle {
            second_level_order_weight: DashMap::new(),
            second_level_order_basic_weight_window: WeightWindow::new(100, 10, WindowUnit::Second),
            day_level_order_weight: DashMap::new(),
            day_level_order_basic_weight_window: WeightWindow::new(200000, 1, WindowUnit::Day),
            minute_level_order_weight: DashMap::new(),
            minute_level_order_basic_weight_window: WeightWindow::new(61000, 5, WindowUnit::Minute),
        }
    }


    fn init_uid_rate(&mut self, uid: u64) {
        if self.second_level_order_weight.contains_key(&uid) && self.minute_level_order_weight.contains_key(&uid) && self.day_level_order_weight.contains_key(&uid) {
            return;
        }
        self.second_level_order_weight.entry(uid).or_insert(self.second_level_order_basic_weight_window.clone());
        self.minute_level_order_weight.entry(uid).or_insert(self.minute_level_order_basic_weight_window.clone());
        self.day_level_order_weight.entry(uid).or_insert(self.day_level_order_basic_weight_window.clone());
    }


    fn second_level_check(&mut self, uid: u64) -> bool {
        self.second_level_order_weight.get_mut(&uid).unwrap().check_weight(1)
    }

    fn minute_level_check(&mut self, uid: u64) -> bool {
        self.minute_level_order_weight.get_mut(&uid).unwrap().check_weight(1)
    }

    fn day_level_check(&mut self, uid: u64) -> bool {
        self.day_level_order_weight.get_mut(&uid).unwrap().check_weight(1)
    }

    pub fn available(&mut self, rate_type: Option<RateType>) -> bool {
        if rate_type.is_none() {
            return true;
        }
        let rate_type = rate_type.unwrap();
        let uid = rate_type.get_uid().unwrap();

        self.init_uid_rate(uid);

        if self.second_level_check(uid) && self.minute_level_check(uid) && self.day_level_check(uid) {
            return true;
        }
        false
    }
}
