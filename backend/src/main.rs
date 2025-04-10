#[macro_use]
extern crate log;
extern crate simplelog;

use axum::{
    extract::{Query, State},
    http::{HeaderValue, Method, StatusCode},
    response::Json,
    routing::get,
    Router,
};
use clap::Parser;
use serde::Deserialize;
use simplelog::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod fetch;
mod models;

use time::macros::format_description;

use models::api_output::{FeedItemResponse, PaginatedResponse};
use models::filters::FeedItemFilters;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,
}

async fn create_db_pool() -> Result<PgPool, Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL environment variable not set. Exiting");
        std::process::exit(1);
    });

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

async fn run_fetch_loop(db_pool: PgPool) {
    let fetch_interval = Duration::from_secs(30);

    info!(
        "Starting fetch loop with interval of {:?} seconds",
        fetch_interval.as_secs()
    );
    loop {
        match fetch::fetch_feed_items().await {
            Ok(fetched_items) => {
                debug!(
                    "Fetched {} items. Processing for new entries...",
                    fetched_items.len()
                );

                if !fetched_items.is_empty() {
                    match db::insert_feed_items_batch(&db_pool, &fetched_items).await {
                        Ok(_) => {
                            debug!(
                                "Successfully inserted batch of {} items into the database.",
                                fetched_items.len()
                            );
                        }
                        Err(e) => {
                            error!("Error inserting batch into the database: {}", e);
                        }
                    }
                } else {
                    debug!("No items found in this fetch.");
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
    #[serde(flatten)]
    filters: FeedItemFilters,
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<i64>,
    limit: Option<i64>,
    search: Option<String>,
}

#[axum::debug_handler]
async fn feed_items_handler(
    State(db_pool): State<PgPool>,
    Query(params): Query<FeedQueryParams>,
) -> Result<Json<PaginatedResponse<FeedItemResponse>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let filters = params.filters;

    let items_result = db::get_feed_items_with_filters(&db_pool, &filters, limit, offset).await;

    let total_result = db::get_total_count_with_filters(&db_pool, &filters).await;

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
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<String>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50);
    let search = params.search;

    match db::get_brands_paginated(&db_pool, page, limit, search).await {
        Ok((brands, total)) => {
            let total_pages = (total + limit - 1) / limit;
            Ok(Json(PaginatedResponse {
                items: brands,
                total,
                page,
                total_pages,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
async fn cities_handler(
    State(db_pool): State<PgPool>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<String>>, StatusCode> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(50);
    let search = params.search;

    match db::get_cities_paginated(&db_pool, page, limit, search).await {
        Ok((cities, total)) => {
            let total_pages = (total + limit - 1) / limit;
            Ok(Json(PaginatedResponse {
                items: cities,
                total,
                page,
                total_pages,
            }))
        }
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

    let term_config = ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            log_level,
            term_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            term_config,
            std::fs::File::create("live-feed.log").unwrap(),
        ),
    ])
    .unwrap();

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

    let allowed_origins =
        std::env::var("ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .map(|s| s.trim().parse::<HeaderValue>().unwrap())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/feed", get(feed_items_handler))
        .route("/brands", get(brands_handler))
        .route("/cities", get(cities_handler))
        .with_state(db_pool)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3133").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
