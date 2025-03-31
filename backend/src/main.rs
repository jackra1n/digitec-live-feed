#[macro_use] extern crate log;
extern crate simplelog;

use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error};
use tokio::time::sleep;
use simplelog::*;
use clap::Parser;
use axum::{routing::get, Router, response::Json, extract::State, http::StatusCode};

mod fetch;
mod db;
mod feed_cache;
mod types;

use feed_cache::FeedItemCache;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,
}

async fn create_db_pool() -> Result<PgPool, Error> {
    let db_url = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| {
        error!("DATABASE_URL environment variable not set. Exiting");
        std::process::exit(1);
    });

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

async fn run_fetch_loop(db_pool: PgPool) {
    let mut cache = FeedItemCache::new(50);
    let fetch_interval = Duration::from_secs(30);

    info!("Starting fetch loop with interval of {:?} seconds", fetch_interval.as_secs());
    loop {
        match fetch::fetch_feed_items().await {
            Ok(fetched_items) => {
                debug!("Fetched {} items. Processing for new entries...", fetched_items.len());
                
                let new_items = cache.process_and_filter_new(fetched_items);
                debug!("New items after filtering: {}", new_items.len());

                if !new_items.is_empty() {
                    match db::insert_feed_items_batch(&db_pool, &new_items).await {
                        Ok(_) => {
                            debug!("Successfully inserted batch of {} items into the database.", new_items.len());
                        }
                        Err(e) => {
                            error!("Error inserting batch into the database: {}", e);
                        }
                    }
                } else {
                    debug!("No new items found in this fetch.");
                }
            }
            Err(e) => {
                error!("Failed to fetch feed items: {}", e);
            }
        }

        debug!("Waiting for {:?} before next fetch.", fetch_interval);
        sleep(fetch_interval).await;
    }
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

    dotenvy::dotenv().ok();

    let db_pool = match create_db_pool().await {
        Ok(pool) => {
            info!("Database connection pool created successfully.");
            pool
        }
        Err(e) => {
            error!("Failed to create database connection pool: {:?}", e);
            std::process::exit(1);
        }
    };

    let fetch_pool = db_pool.clone();
    tokio::spawn(async move {
        run_fetch_loop(fetch_pool).await;
    });
    info!("Background fetch loop spawned");

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/last-items", get(latest_feed_items_handler))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3133").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[axum::debug_handler]
async fn latest_feed_items_handler(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<types::FeedItem>>, StatusCode> {
    let items_result = db::get_latest_feed_items(&db_pool, 6).await;

    match items_result {
        Ok(items) => {
            debug!("Successfully fetched {} items from DB", items.len());
            Ok(Json(items))
        }
        Err(e) => {
            error!("Database error in latest_feed_items_handler: {:?}", e); 
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}