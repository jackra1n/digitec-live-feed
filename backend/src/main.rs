#[macro_use] extern crate log;
extern crate simplelog;

use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error};
use tokio::time::sleep;
use simplelog::*;
use clap::Parser;
use axum::{
    routing::get,
    Router,
    response::Json,
    extract::{State, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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

#[derive(Deserialize)]
struct FeedQueryParams {
    page: Option<i64>,
    limit: Option<i64>,
    transaction_type: Option<i32>,
    brand: Option<String>,
    city: Option<String>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    search: Option<String>,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i64,
    total_pages: i64,
}

#[axum::debug_handler]
async fn feed_items_handler(
    State(db_pool): State<PgPool>,
    Query(params): Query<FeedQueryParams>,
) -> Result<Json<PaginatedResponse<types::FeedItem>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let items_result = db::get_feed_items_with_filters(
        &db_pool,
        limit,
        offset,
        params.transaction_type,
        params.brand.clone(),
        params.city.clone(),
        params.start_date,
        params.end_date,
        params.search.clone(),
    ).await;

    let total_result = db::get_total_count_with_filters(
        &db_pool,
        params.transaction_type,
        params.brand,
        params.city,
        params.start_date,
        params.end_date,
        params.search,
    ).await;

    match (items_result, total_result) {
        (Ok(items), Ok(total)) => {
            let total_pages = (total + limit - 1) / limit;
            Ok(Json(PaginatedResponse {
                items,
                total,
                page,
                total_pages,
            }))
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
async fn brands_handler(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<String>>, StatusCode> {
    match db::get_unique_brands(&db_pool).await {
        Ok(brands) => Ok(Json(brands)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
async fn cities_handler(
    State(db_pool): State<PgPool>,
) -> Result<Json<Vec<String>>, StatusCode> {
    match db::get_unique_cities(&db_pool).await {
        Ok(cities) => Ok(Json(cities)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
        .route("/feed", get(feed_items_handler))
        .route("/brands", get(brands_handler))
        .route("/cities", get(cities_handler))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3133").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}