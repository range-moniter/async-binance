use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum WindowUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl WindowUnit {
    pub fn calculate_window_timestamp(&self, window_interval: u64) -> u64 {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        match self {
            WindowUnit::Second => current_time + window_interval,
            WindowUnit::Minute => current_time - window_interval * 60,
            WindowUnit::Hour => current_time + window_interval * 60 * 60,
            WindowUnit::Day => current_time + window_interval * 60 * 60 * 24,
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct WeightWindow {
    weight_window: Arc<Mutex<(u32, u64)>>,
    basic_weight: u32,
    interval: u64,
    unit: WindowUnit
}

impl WeightWindow {
    // window_interval: the basic size about slide window size;
    // weight: the current window weight
    pub fn new(weight: u32, interval: u64, unit: WindowUnit) -> Self {
        WeightWindow {
            weight_window: Arc::new(Mutex::new((weight, unit.calculate_window_timestamp(interval)))),
            basic_weight: weight,
            interval,
            unit
        }
    }

    pub fn check_weight(&mut self, weight: u32) -> bool {
        let mut guard = self.weight_window.lock().unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if current_time > guard.1 {
            *guard = (
                self.basic_weight - weight,
                self.unit.calculate_window_timestamp(self.interval)
            );
            true
        } else {
            if guard.0 >= weight {
                *guard = (guard.0 - weight, guard.1);
                true
            } else {
                false
            }
        }
    }
}

