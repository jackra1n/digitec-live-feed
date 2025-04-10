use super::serde_helpers;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiFeedItem {
    #[serde(with = "serde_helpers::string_to_i32")]
    pub id: i32,
    pub user_name: String,
    pub city_name: Option<String>,
    pub date_time: DateTime<Utc>,
    pub image_url: Option<String>,
    pub brand_name: Option<String>,
    #[serde(with = "serde_helpers::trim_opt_string")]
    pub full_product_name: Option<String>,
    pub display_price: Option<ApiDisplayPrice>,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiDisplayPrice {
    #[serde(serialize_with = "serde_helpers::serialize_decimal_as_string")]
    pub amount_inclusive: Decimal,
    #[serde(serialize_with = "serde_helpers::serialize_decimal_as_string")]
    pub amount_exclusive: Decimal,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SocialShopping {
    #[allow(dead_code)]
    latest_transaction_time_stamp: Option<String>,
    pub items: Vec<ApiFeedItem>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "socialShopping")]
    pub social_shopping: SocialShopping,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: Data,
}

impl GraphQLResponse {
    pub fn into_items(self) -> Vec<ApiFeedItem> {
        self.data.social_shopping.items
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &Vec<ApiFeedItem> {
        &self.data.social_shopping.items
    }
}
