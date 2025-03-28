use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER, ACCEPT_ENCODING, ACCEPT_LANGUAGE};
use reqwest::Error;
use http;

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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")] // good practice for JSON APIs
pub struct FeedItem {
    // use Option<> liberally for fields that might be missing or null
    pub id: Option<String>,
    pub user_name: Option<String>,
    pub city_name: Option<String>,
    pub date_time: Option<String>,
    pub image_url: Option<String>,
    pub brand_name: Option<String>,
    // if full_product_name might ever be missing, make it Option<String>
    pub full_product_name: String,
    pub display_price: Option<DisplayPrice>,
    #[serde(rename = "oAuthProviderName")] // keep explicit renames where needed
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
    // use Option<f64> if these amounts might be missing/null
    pub amount_inclusive: f64,
    pub amount_exclusive: f64,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SocialShopping {
    #[allow(dead_code)]
    latest_transaction_time_stamp: Option<String>,
    items: Vec<FeedItem>,
}

#[derive(Deserialize, Debug)]
struct Data {
    #[serde(rename = "socialShopping")]
    social_shopping: SocialShopping,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    data: Data,
    // errors: Option<serde_json::Value>, // uncomment if GraphQL errors might be present
}

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
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let request_payload = GraphQLRequest {
        operation_name: "GET_SOCIAL_SHOPPINGS",
        variables: Variables { take: 6, latest: None },
        query: GRAPHQL_QUERY,
    };

    println!("Sending Request...");

    let response = client.post(DIGITEC_URL)
        .json(&[request_payload])
        .send()
        .await?;

    println!("Response Status: {}", response.status());

    let response = response.error_for_status()?;

    let raw_body = response.text().await?;
    println!("--- Raw Response Body Start ---");
    println!("{}", raw_body);
    println!("--- Raw Response Body End ---");

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

    println!("Successfully parsed JSON.");

    Ok(response_vec.get(0)
        .map(|resp| resp.data.social_shopping.items.clone())
        .unwrap_or_default())
}