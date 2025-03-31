#![allow(dead_code)]
use serde::{Serialize, Deserialize, Serializer};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

mod string_to_i32 {
    use serde::{self, Deserialize, Deserializer};
    use std::str::FromStr;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i32::from_str(&s).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }
}

mod trim_opt_string {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_s: Option<String> = Option::deserialize(deserializer)?;

        match opt_s {
            Some(s) => Ok(Some(s.trim_start().to_string())),
            None => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    #[serde(with = "string_to_i32")]
    pub id: i32,
    pub user_name: String,
    pub city_name: Option<String>,
    pub date_time: DateTime<Utc>,
    pub image_url: Option<String>,
    pub brand_name: Option<String>,
    #[serde(with = "trim_opt_string")]
    pub full_product_name: Option<String>,
    pub display_price: Option<DisplayPrice>,
    #[serde(rename = "oAuthProviderName")]
    pub o_auth_provider_name: Option<String>,
    pub target_user_name: Option<String>,
    pub quote: Option<String>,
    pub vote_type_id: Option<i32>,
    pub product_type_name: Option<String>,
    pub social_shopping_transaction_type_id: i32,
    pub url: String,
    pub rating: Option<i32>,
    pub search_string: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPrice {
    #[serde(serialize_with = "serialize_decimal_as_string")]
    pub amount_inclusive: Decimal,
    #[serde(serialize_with = "serialize_decimal_as_string")]
    pub amount_exclusive: Decimal,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SocialShopping {
    #[allow(dead_code)]
    latest_transaction_time_stamp: Option<String>,
    items: Vec<FeedItem>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "socialShopping")]
    pub social_shopping: SocialShopping,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: Data,
    // errors: Option<serde_json::Value>,
}

impl GraphQLResponse {
    pub fn into_items(self) -> Vec<FeedItem> {
        self.data.social_shopping.items
    }

    pub fn items(&self) -> &Vec<FeedItem> {
        &self.data.social_shopping.items
    }
}