use serde::Serialize;
use ureq::Error;
use crate::types::live_feed::{FeedItem, GraphQLResponse};

const DIGITEC_URL: &str = "https://www.digitec.ch/api/graphql/get-social-shoppings";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1";

#[derive(Serialize)]
struct GraphQLRequest {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    query: &'static str,
    variables: Variables,
}

#[derive(Serialize)]
struct Variables {
    take: i32,
    latest: Option<String>,
}

const GRAPHQL_QUERY: &str = r#"query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {
  socialShopping(take: $take, latest: $latest) {
    latestTransactionTimeStamp
    items {
      id
      userName
      cityName
      dateTime
      imageUrl
      brandName
      fullProductName
      displayPrice {
        amountInclusive
        amountExclusive
        currency
      }
      oAuthProviderName
      targetUserName
      quote
      voteTypeId
      productTypeName
      socialShoppingTransactionTypeId
      url
      rating
      searchString
    }
  }
}"#;


pub fn fetch_feed_items() -> Result<Vec<FeedItem>, Error> {
    let request = GraphQLRequest {
        operation_name: "GET_SOCIAL_SHOPPINGS",
        query: GRAPHQL_QUERY,
        variables: Variables {
            take: 6,
            latest: None,
        },
    };

    let response: Vec<GraphQLResponse> = ureq::post(DIGITEC_URL)
        .header("Accept", "*/*")
        .header("User-Agent", USER_AGENT) 
        .header("Content-Type", "application/json")
        .header("Origin", "https://www.digitec.ch")
        .header("Referer", "https://www.digitec.ch/")
        .send_json(&[request])?
        .body_mut()
        .read_json()?;
    
    Ok(response.get(0)
        .map(|resp| resp.data.socialShopping.items.clone())
        .unwrap_or_default())
}