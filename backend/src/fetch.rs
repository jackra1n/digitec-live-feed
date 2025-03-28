use serde::Serialize;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, ACCEPT_ENCODING, ACCEPT_LANGUAGE};
use reqwest::Error;
use http;
use crate::types::{FeedItem, GraphQLResponse};

const DIGITEC_URL: &str = "https://www.digitec.ch/api/graphql/get-social-shoppings";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1";

#[derive(Serialize, Debug, Clone)]
struct GraphQLRequest {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    variables: Variables,
    query: &'static str,
}

#[derive(Serialize, Debug, Clone)]
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

pub async fn fetch_feed_items_reqwest() -> Result<Vec<FeedItem>, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ORIGIN, "https://www.digitec.ch".parse().unwrap());
    headers.insert(REFERER, "https://www.digitec.ch/en/daily-deal".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(headers)
        .use_native_tls() // use system's native TLS
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let request_payload = GraphQLRequest {
        operation_name: "GET_SOCIAL_SHOPPINGS",
        variables: Variables { take: 6, latest: None },
        query: GRAPHQL_QUERY,
    };

    let response = client.post(DIGITEC_URL)
        .json(&[request_payload])
        .send()
        .await?;

    let response = response.error_for_status()?;
    let raw_body = response.text().await?;

    let response_vec: Vec<GraphQLResponse> = match serde_json::from_str(&raw_body) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("JSON Parsing Error: {}", e);
            eprintln!("Failed to parse the raw body shown above.");
            let error_response = reqwest::Response::from(
                http::Response::builder()
                    .status(http::StatusCode::OK)
                    .body(raw_body)
                    .unwrap(),
            );
            return Err(error_response.json::<()>().await.unwrap_err());
        }
    };

    Ok(response_vec.into_iter()
        .next()
        .map(|resp| resp.into_items().clone())
        .unwrap_or_default())
}