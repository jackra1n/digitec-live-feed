#![allow(dead_code)]
use rust_decimal::Decimal;
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(Debug, Clone, FromRow)]
pub struct NameIdPair {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Brand {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ProductType {
    pub id: i32,
    pub name: String,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CanonicalCity {
    pub id: i32,
    #[sqlx(rename = "canonicalName")]
    pub canonical_name: String,
    #[sqlx(rename = "isSwiss")]
    pub is_swiss: bool,
    pub canton: Option<String>,
    #[sqlx(rename = "countryCode")]
    pub country_code: Option<String>,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct RawCityNameMap {
    #[sqlx(rename = "rawInput")]
    pub raw_input: String,
    #[sqlx(rename = "canonicalCityId")]
    pub canonical_city_id: Option<i32>,
    #[sqlx(rename = "mappingStatus")]
    pub mapping_status: String,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct SocialShoppingItem {
    pub id: i64,
    #[sqlx(rename = "api_id")]
    pub api_id: i64,
    #[sqlx(rename = "userName")]
    pub user_name: String,
    #[sqlx(rename = "cityName")]
    pub city_name: Option<String>,
    #[sqlx(rename = "dateTime")]
    pub date_time: DateTime<Utc>,
    #[sqlx(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[sqlx(rename = "brandId")]
    pub brand_id: Option<i32>,
    #[sqlx(rename = "fullProductName")]
    pub full_product_name: Option<String>,
    #[sqlx(rename = "oAuthProviderName")]
    pub o_auth_provider_name: Option<String>,
    #[sqlx(rename = "targetUserName")]
    pub target_user_name: Option<String>,
    pub quote: Option<String>,
    #[sqlx(rename = "voteTypeId")]
    pub vote_type_id: Option<i32>,
    #[sqlx(rename = "productTypeId")]
    pub product_type_id: Option<i32>,
    #[sqlx(rename = "socialShoppingTransactionTypeId")]
    pub social_shopping_transaction_type_id: i32,
    pub url: String,
    pub rating: Option<i32>,
    #[sqlx(rename = "searchString")]
    pub search_string: Option<String>,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct DisplayPrice {
    pub id: i32,
    #[sqlx(rename = "socialShoppingItemId")]
    pub social_shopping_item_id: i64,
    #[sqlx(rename = "amountInclusive")]
    pub amount_inclusive: Decimal,
    #[sqlx(rename = "amountExclusive")]
    pub amount_exclusive: Decimal,
    pub currency: String,
    #[sqlx(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
}
