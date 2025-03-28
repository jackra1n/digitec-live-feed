mod fetch;
use fetch::fetch_feed_items_reqwest;
mod types;

#[tokio::main]
async fn main() {
    match fetch_feed_items_reqwest().await {
        Ok(items) => {
            println!("Fetched {} items.", items.len());
        }
        Err(e) => {
            eprintln!("Request failed: {}", e);
            if e.is_decode() {
                eprintln!("Specific error was related to decoding the response body.");
            } else if let Some(status) = e.status() {
                eprintln!("HTTP status error: {}", status);
            }
        }
    }
}