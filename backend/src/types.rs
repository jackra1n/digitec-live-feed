#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    pub id: String,
    pub user_name: String,
    pub city_name: Option<String>,
    pub date_time: String,
    pub image_url: Option<String>,
    pub brand_name: Option<String>,
    pub full_product_name: Option<String>,
    pub display_price: Option<DisplayPrice>,
    #[serde(rename = "oAuthProviderName")]
    pub o_auth_provider_name: Option<String>,
    pub target_user_name: Option<String>,
    pub quote: Option<String>,
    pub vote_type_id: Option<i32>,
    pub product_type_name: Option<String>,
    pub social_shopping_transaction_type_id: Option<i32>,
    pub url: Option<String>,
    pub rating: Option<f32>,
    pub search_string: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisplayPrice {
    pub amount_inclusive: f64,
    pub amount_exclusive: f64,
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