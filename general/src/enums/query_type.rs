use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum QueryType {
    FULL,
    MINI,
}