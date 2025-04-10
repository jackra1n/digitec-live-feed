// src/models/serde_helpers.rs

use rust_decimal::Decimal;
use serde::{self, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub mod string_to_i32 {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i32::from_str(&s).map_err(serde::de::Error::custom)
    }

    #[allow(dead_code)]
    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

pub mod trim_opt_string {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_s: Option<String> = Option::deserialize(deserializer)?;

        match opt_s {
            Some(s) => Ok(Some(s.trim_start().to_string()).filter(|s| !s.is_empty())),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(s) => serializer.serialize_some(&s.trim_start()),
            None => serializer.serialize_none(),
        }
    }
}

pub fn serialize_decimal_as_string<S>(decimal: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = decimal.round_dp(2).to_string();
    serializer.serialize_str(&formatted)
}
