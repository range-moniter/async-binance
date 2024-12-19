use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Level {
    L1 = 5,
    L2 = 10,
    L3 = 20,
}

impl Level {
    pub fn value(&self) -> i32 {
        *self as i32
    }
}
