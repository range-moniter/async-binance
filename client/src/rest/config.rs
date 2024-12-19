use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub struct WeightWindow {
    window_weight: u16,
    window_size: u64
}

impl WeightWindow {
    pub fn new(window_weight: u16, window_size: u64) -> Self {
        WeightWindow {
            window_size, window_weight
        }
    }
    pub fn window_weight(&self) -> u16 {
        self.window_weight
    }
    pub fn window_size(&self) -> u64 {
        self.window_size
    }
}
pub struct Config {
    request_timeout: Duration,
    weight_window_config: WeightWindow,
}

impl Config {
    pub fn new_default() -> Self {
        Config {
            request_timeout: Duration::from_secs(5),
            weight_window_config: WeightWindow::new(6000,1),
        }
    }
    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }
    pub fn weight_window_config(&self) -> WeightWindow {
        self.weight_window_config
    }
}