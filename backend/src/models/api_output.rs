use super::serde_helpers;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedItemResponse {
    pub id: i64,
    pub user_name: String,
    pub city_name: Option<String>,
    pub canton: Option<String>,
    pub country_code: Option<String>,
    pub date_time: DateTime<Utc>,
    pub image_url: Option<String>,
    pub brand_name: Option<String>,
    pub full_product_name: Option<String>,
    pub o_auth_provider_name: Option<String>,
    pub target_user_name: Option<String>,
    pub quote: Option<String>,
    pub vote_type_id: Option<i32>,
    pub product_type_name: Option<String>,
    pub social_shopping_transaction_type_id: i32,
    pub url: String,
    pub rating: Option<i32>,
    pub search_string: Option<String>,
    pub display_price: Option<DisplayPriceResponse>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPriceResponse {
    #[serde(serialize_with = "serde_helpers::serialize_decimal_as_string")]
    pub amount_inclusive: Decimal,
    #[serde(serialize_with = "serde_helpers::serialize_decimal_as_string")]
    pub amount_exclusive: Decimal,
    pub currency: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub total_pages: i64,
}
