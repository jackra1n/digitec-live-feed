use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Default, Clone, Deserialize)]
pub struct FeedItemFilters {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub transaction_type: Option<i32>,
    pub brand_name: Option<String>,
    pub city_name: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub search: Option<String>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct FeedQueryParams {
    #[serde(flatten)]
    pub filters: FeedItemFilters,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub page: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub limit: Option<i64>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub page: Option<i64>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub limit: Option<i64>,
    pub search: Option<String>,
}