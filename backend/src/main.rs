#[macro_use] extern crate log;
extern crate simplelog;

use std::time::Duration;
use feed_cache::FeedItemCache;
use tokio::time::sleep;
use simplelog::*;

mod fetch;
mod feed_cache;
mod types;

#[tokio::main]
async fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), std::fs::File::create("live-feed.log").unwrap()),
        ]
    ).unwrap();

    let mut cache = FeedItemCache::new(50);
    let fetch_interval = Duration::from_secs(30);

    info!("Fetching feed items...");
    loop {
        match fetch::fetch_feed_items_reqwest().await {
            Ok(items) => {
                debug!("Fetched {} items. Processing new ones...", items.len());
                let mut new_item_count = 0;
                for item in items {
                    if cache.add_and_check(&item) {
                        new_item_count += 1;
                        debug!(
                            "  -> New Item: {:?} (ID: {:?})",
                            item.full_product_name,
                            item.id
                        );
                    }
                }
                debug!("Processed {} new items. Cache size: {}", new_item_count, cache.len());
            }
            Err(e) => {
                error!("Error fetching feed items: {}", e);
            }
        }

        debug!("Waiting for {:?} before next fetch...", fetch_interval);
        sleep(fetch_interval).await;
        debug!("----------------------------------------");
    }
}