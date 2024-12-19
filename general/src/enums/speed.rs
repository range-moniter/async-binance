use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Speed {
    Ms1000,
    Ms100,
}

impl Speed {
    pub fn as_str(&self) -> &str {
        match self {
            Speed::Ms100 => "100ms",
            Speed::Ms1000 => "1000ms",
        }
    }
}