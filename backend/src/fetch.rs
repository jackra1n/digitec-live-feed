use serde::Serialize;
use ureq::Error;
use crate::types::live_feed::{FeedItem, GraphQLResponse};

const DIGITEC_URL: &str = "https://www.digitec.ch/api/graphql/get-social-shoppings";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1";

#[derive(Serialize, Debug)]
struct GraphQLRequest {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    variables: Variables,
    query: &'static str,
}

#[derive(Serialize, Debug)]
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
        variables: Variables {
            take: 6,
            latest: None,
        },
        query: GRAPHQL_QUERY,
    };

    println!("Request: {:#?}", request);

    let raw_response = ureq::post(DIGITEC_URL)
        .header("accept", "*/*")
        .header("user-agent", USER_AGENT) 
        .header("content-type", "application/json")
        .header("origin", "https://www.digitec.ch")
        .header("referer", "https://www.digitec.ch/en/daily-deal")
        .header("sec-fetch-mode", "cors")
        .send_json(&[request])?
        .body_mut()
        .read_to_string()?;

    let response: Vec<GraphQLResponse> = serde_json::from_str(&raw_response)?;
    
    Ok(response.get(0)
        .map(|resp| resp.data.socialShopping.items.clone())
        .unwrap_or_default())
}