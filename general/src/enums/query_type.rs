use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum QueryType {
    FULL,
    MINI,
}