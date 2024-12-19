use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone, Copy , Eq, PartialEq, Hash)]
pub enum Interval {
    #[serde(rename = "1s")]
    Second1,
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "3m")]
    Minute3,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "2h")]
    Hour2,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "6h")]
    Hour6,
    #[serde(rename = "8h")]
    Hour8,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "3d")]
    Day3,
    #[serde(rename = "1w")]
    Week,
    #[serde(rename = "1M")]
    Month,
}


impl Interval {
    pub fn as_str(&self) -> &str {
        match self {
            Interval::Second1 => "1s",
            Interval::Minute1 => "1m",
            Interval::Minute3 => "3m",
            Interval::Minute5 => "5m",
            Interval::Minute15 => "15m",
            Interval::Minute30 => "30m",
            Interval::Hour1 => "1h",
            Interval::Hour2 => "2h",
            Interval::Hour4 => "4h",
            Interval::Hour6 => "6h",
            Interval::Hour8 => "8h",
            Interval::Hour12 => "12h",
            Interval::Day1 => "1d",
            Interval::Day3 => "3d",
            Interval::Week => "1w",
            Interval::Month => "1M",
        }
    }
}
