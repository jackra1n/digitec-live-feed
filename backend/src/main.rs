use std::time::Duration;
use feed_cache::FeedItemCache;
use tokio::time::sleep;

mod fetch;
mod feed_cache;
mod types;

#[tokio::main]
async fn main() {
    let mut cache = FeedItemCache::new(50);
    let fetch_interval = Duration::from_secs(30);

    loop {
        println!("Fetching feed items...");
        match fetch::fetch_feed_items_reqwest().await {
            Ok(items) => {
                println!("Fetched {} items. Processing new ones...", items.len());
                let mut new_item_count = 0;
                for item in items {
                    if cache.add_and_check(&item) {
                        new_item_count += 1;
                        println!(
                            "  -> New Item: {:?} (ID: {:?})",
                            item.full_product_name,
                            item.id
                        );
                    }
                }
                println!("Processed {} new items. Cache size: {}", new_item_count, cache.len());
            }
            Err(e) => {
                eprintln!("Error fetching feed items: {}", e);
            }
        }

        println!("Waiting for {:?} before next fetch...", fetch_interval);
        sleep(fetch_interval).await;
        println!("----------------------------------------");
    }
}