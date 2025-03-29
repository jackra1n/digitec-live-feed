use serde::Serialize;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, ACCEPT_ENCODING, ACCEPT_LANGUAGE};
use reqwest::Error;
use crate::types::{FeedItem, GraphQLResponse};

const DIGITEC_URL: &str = "https://www.digitec.ch/api/graphql/get-social-shoppings";
const USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36";

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

fn create_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ORIGIN, "https://www.digitec.ch".parse().unwrap());
    headers.insert(REFERER, "https://www.digitec.ch/en/daily-deal".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    // additional headers, not really needed
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, br".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());
    headers
}

fn create_client(headers: HeaderMap) -> Result<reqwest::Client, reqwest::Error> {
    return reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(headers)
        .use_rustls_tls() 
        .timeout(std::time::Duration::from_secs(10))
        .build();
}

pub async fn fetch_feed_items_reqwest() -> Result<Vec<FeedItem>, Error> {
    let headers = create_headers();
    let client = create_client(headers) 
        .map_err(|e| {
            error!("Error creating HTTP client: {:?}", e);
            e
        })?;

    let request_payload = GraphQLRequest {
        operation_name: "GET_SOCIAL_SHOPPINGS",
        variables: Variables { take: 10, latest: None },
        query: GRAPHQL_QUERY,
    };

    let response = client.post(DIGITEC_URL)
        .json(&[request_payload])
        .send()
        .await?;

    let response = response.error_for_status()?;
    let raw_body = response.text().await?;

    trace!("Raw Response Body: {:?}", raw_body);

    let response_vec: Vec<GraphQLResponse> = match serde_json::from_str(&raw_body) {
        Ok(parsed) => parsed,
        Err(e) => {
            warn!("JSON Parsing Error: {:?}", e);
            warn!("Response Body: {:?}", raw_body);
            panic!("Failed to parse GraphQL JSON: {:?}", e);
        }
    };

    Ok(response_vec.into_iter()
        .next()
        .map(|resp| resp.into_items().clone())
        .unwrap_or_default())
}