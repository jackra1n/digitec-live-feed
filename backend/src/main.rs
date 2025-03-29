#[macro_use] extern crate log;
extern crate simplelog;

use std::time::Duration;
use feed_cache::FeedItemCache;
use tokio::time::sleep;
use simplelog::*;
use clap::Parser;

mod fetch;
mod feed_cache;
mod types;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let log_level = if cli.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    CombinedLogger::init(
        vec![
            TermLogger::new(log_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), std::fs::File::create("live-feed.log").unwrap()),
        ]
    ).unwrap();

    debug!("Debug logging enabled");

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
                            "  -> New Item: {:?} (ID: {:?}), Date: {:?}",
                            item.full_product_name.as_deref().unwrap_or("N/A"),
                            item.id,
                            item.date_time,
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