#![allow(non_snake_case, dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: SocialShoppingData,
}

#[derive(Deserialize, Debug)]
pub struct SocialShoppingData {
    pub socialShopping: SocialShopping,
}

#[derive(Deserialize, Debug)]
pub struct SocialShopping {
    pub latestTransactionTimeStamp: String,
    pub items: Vec<FeedItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FeedItem {
    pub id: String,
    pub userName: String,
    pub cityName: Option<String>,
    pub dateTime: String,
    pub imageUrl: Option<String>,
    pub brandName: Option<String>,
    pub fullProductName: Option<String>,
    pub displayPrice: Option<DisplayPrice>,
    pub oAuthProviderName: Option<String>,
    pub targetUserName: Option<String>,
    pub quote: Option<String>,
    pub voteTypeId: Option<i32>,
    pub productTypeName: Option<String>,
    pub socialShoppingTransactionTypeId: i32,
    pub url: String,
    pub rating: Option<f32>,
    pub searchString: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DisplayPrice {
    pub amountInclusive: f64,
    pub amountExclusive: f64,
    pub currency: String,
}