use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serializer};

pub fn from_str_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u32::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn from_str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u64::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn from_str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f32::from_str(&s).map_err(serde::de::Error::custom)
}


pub fn from_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(serde::de::Error::custom)
}


pub fn serialize_option_vec<S, T>(items: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    match items {
        None => serializer.serialize_none(),
        Some(items) => {
            let result = items.iter()
                .map(|x| format!("\"{}\"", x.to_string()))
                .collect::<Vec<String>>()
                .join(",");
            serializer.serialize_str(format!("[{}]", result).as_str())
        }
    }
}