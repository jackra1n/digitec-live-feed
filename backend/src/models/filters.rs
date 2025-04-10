use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct FeedItemFilters {
    pub transaction_type: Option<i32>,
    pub brand_name: Option<String>,
    pub city_name: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub search: Option<String>,
}
