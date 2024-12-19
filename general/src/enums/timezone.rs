use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Timezone {
    UTC8,
}

impl Timezone {
    pub fn as_str(&self) -> &str {
        match self {
            Timezone::UTC8 => "+08:00",
        }
    }
}