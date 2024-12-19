use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum WindowSize {
    OneHour,
    FourHours,
    OneDay,
}


impl WindowSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneHour => "1h",
            Self::OneDay => "4h",
            Self::FourHours => "1d",
        }
    }
}