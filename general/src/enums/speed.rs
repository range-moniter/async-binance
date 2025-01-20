use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Speed {
    Ms500,
    Ms100,
}

impl Speed {
    pub fn as_str(&self) -> &str {
        match self {
            Speed::Ms100 => "100ms",
            Speed::Ms500 => "500ms",
        }
    }
}