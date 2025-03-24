use reqwest::{self, Error, Response};
use serde::Serialize;

const DIGITEC_FETCH_URL: &str = "https://www.digitec.ch/api/graphql/get-social-shoppings";

#[derive(Debug)]
#[derive(Serialize)]
struct Variables {
    take: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize)]
struct GraphQLRequest {
    operation_name: &'static str,
    query: &'static str,
    variables: Variables,
}

const GRAPHQL_QUERY: &str = r#"
query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {
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
}
"#;


fn create_headers() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "*/*".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Origin", "https://www.digitec.ch".parse().unwrap());
    headers.insert("Referer", "https://www.digitec.ch/en/daily-deal".parse().unwrap());
    headers
}


pub async fn fetch_items() -> Result<Response, Error> {
    let headers = create_headers();

    let request_body =  GraphQLRequest {
        operation_name: "GET_SOCIAL_SHOPPINGS",
        query: GRAPHQL_QUERY,
        variables: Variables {
            take: 6,
            latest: None,
        },
    };

    println!("{:#?}", request_body);

    let client = reqwest::Client::new();
    let resp = client.post(DIGITEC_FETCH_URL)
        .json(&request_body)
        .headers(headers)
        .send()
        .await?;

    println!("{:#?}", resp);

    Ok(resp)

}