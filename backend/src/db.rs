use rust_decimal::Decimal;
use sqlx::{postgres::PgRow, Row, PgPool, Error};
use chrono::{DateTime, Utc};
use crate::types::*;


pub async fn insert_feed_items_batch(pool: &PgPool, items: &[FeedItem]) -> Result<(), Error> {
    if items.is_empty() {
        return Ok(());
    }

    let mut tx = pool.begin().await?;

    for item in items {
        let item_insert_result = sqlx::query!(
            r#"
            INSERT INTO "SocialShoppingItem" (
                id, "userName", "cityName", "dateTime", "imageUrl", "brandName",
                "fullProductName", "oAuthProviderName", "targetUserName", "quote",
                "voteTypeId", "productTypeName", "socialShoppingTransactionTypeId",
                "url", "rating", "searchString"
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            ON CONFLICT (id) DO NOTHING
            "#,
            item.id as i32,
            item.user_name,
            item.city_name,
            item.date_time, // assumes TIMESTAMPTZ column, maps from DateTime<Utc>
            item.image_url,
            item.brand_name,
            item.full_product_name,
            item.o_auth_provider_name,
            item.target_user_name,
            item.quote,
            item.vote_type_id,
            item.product_type_name,
            item.social_shopping_transaction_type_id,
            item.url,
            item.rating,
            item.search_string
        )
        .execute(&mut *tx)
        .await;

        if let Err(e) = item_insert_result {
            log::error!(
                "Error inserting item ID {} within batch transaction: {}. Rolling back batch.",
                item.id,
                e
            );
            return Err(e);
        }

        if let Some(price) = &item.display_price {

            let price_insert_result = sqlx::query!(
                r#"
                INSERT INTO "DisplayPrice" (
                    "socialShoppingItemId", "amountInclusive", "amountExclusive", "currency"
                ) VALUES (
                    $1, $2, $3, $4
                )
                ON CONFLICT ("socialShoppingItemId") DO NOTHING
                "#,
                item.id as i32,
                price.amount_inclusive as Decimal,
                price.amount_exclusive as Decimal,
                price.currency
            )
            .execute(&mut *tx)
            .await;

            if let Err(e) = price_insert_result {
                 log::error!(
                    "Error inserting price for item ID {} within batch transaction: {}. Rolling back batch.",
                    item.id,
                    e
                 );
                return Err(e);
            }
        }
    }

    tx.commit().await?;
    log::debug!("Successfully committed batch insert of {} items.", items.len());

    Ok(())
}

fn map_row_to_feed_item(row: &PgRow) -> Result<FeedItem, sqlx::Error> {
    let amount_inclusive: Option<Decimal> = row.try_get("amount_inclusive")?;
    let amount_exclusive: Option<Decimal> = row.try_get("amount_exclusive")?;
    let currency: Option<String> = row.try_get("currency")?;

    let display_price = match (amount_inclusive, amount_exclusive, currency) {
        (Some(ai), Some(ae), Some(c)) => Some(DisplayPrice {
            amount_inclusive: ai,
            amount_exclusive: ae,
            currency: c,
        }),
        _ => None, // If any part is NULL from the LEFT JOIN, the whole price is None
    };

    let id: i32 = row.try_get("id")?;
    let user_name: String = row.try_get("userName")?;
    let date_time: DateTime<Utc> = row.try_get("dateTime")?;
    let social_shopping_transaction_type_id: i32 = row.try_get("socialShoppingTransactionTypeId")?;
    let url: String = row.try_get("url")?;

    let city_name: Option<String> = row.try_get("cityName")?;
    let image_url: Option<String> = row.try_get("imageUrl")?;
    let brand_name: Option<String> = row.try_get("brandName")?;
    let full_product_name: Option<String> = row.try_get("fullProductName")?;
    let o_auth_provider_name: Option<String> = row.try_get("oAuthProviderName")?;
    let target_user_name: Option<String> = row.try_get("targetUserName")?;
    let quote: Option<String> = row.try_get("quote")?;
    let vote_type_id: Option<i32> = row.try_get("voteTypeId")?;
    let product_type_name: Option<String> = row.try_get("productTypeName")?;
    let rating: Option<i32> = row.try_get("rating")?;
    let search_string: Option<String> = row.try_get("searchString")?;

    let trimmed_full_product_name = full_product_name.map(|s| s.trim_start().to_string());

    Ok(FeedItem {
        id,
        user_name,
        city_name,
        date_time,
        image_url,
        brand_name,
        full_product_name: trimmed_full_product_name,
        display_price,
        o_auth_provider_name,
        target_user_name,
        quote,
        vote_type_id,
        product_type_name,
        social_shopping_transaction_type_id,
        url,
        rating,
        search_string,
    })
}

pub async fn get_latest_feed_items(pool: &PgPool, limit: i64) -> Result<Vec<FeedItem>, Error> {
    let sql = r#"
        SELECT
            ssi.id,
            ssi."userName",
            ssi."cityName",
            ssi."dateTime",
            ssi."imageUrl",
            ssi."brandName",
            ssi."fullProductName",
            ssi."oAuthProviderName",
            ssi."targetUserName",
            ssi.quote,
            ssi."voteTypeId",
            ssi."productTypeName",
            ssi."socialShoppingTransactionTypeId",
            ssi.url,
            ssi.rating,
            ssi."searchString",
            dp."amountInclusive" AS amount_inclusive,
            dp."amountExclusive" AS amount_exclusive,
            dp.currency AS currency
        FROM
            "SocialShoppingItem" ssi
        LEFT JOIN
            "DisplayPrice" dp ON ssi.id = dp."socialShoppingItemId"
        WHERE
            ssi."dateTime" >= (NOW() - INTERVAL '1 day')
        ORDER BY
            ssi."dateTime" DESC
        LIMIT $1
    "#;

    let items = sqlx::query(sql)
        .bind(limit)
        .map(|row: PgRow| map_row_to_feed_item(&row))
        .fetch_all(pool)
        .await?;

    items.into_iter().collect()
}

pub async fn get_feed_items_with_filters(
    pool: &PgPool,
    limit: i64,
    offset: i64,
    transaction_type: Option<i32>,
    brand: Option<String>,
    city: Option<String>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    search_query: Option<String>,
) -> Result<Vec<FeedItem>, Error> {
    let mut conditions = vec!["1=1".to_string()];
    let mut params: Vec<String> = vec![];
    let mut param_pos = 1;

    if let Some(tt) = transaction_type {
        conditions.push(format!("ssi.\"socialShoppingTransactionTypeId\" = ${}", param_pos));
        params.push(tt.to_string());
        param_pos += 1;
    }

    if let Some(b) = brand {
        conditions.push(format!("ssi.\"brandName\" = ${}", param_pos));
        params.push(b);
        param_pos += 1;
    }

    if let Some(c) = city {
        conditions.push(format!("ssi.\"cityName\" = ${}", param_pos));
        params.push(c);
        param_pos += 1;
    }

    if let Some(sd) = start_date {
        conditions.push(format!("ssi.\"dateTime\" >= ${}", param_pos));
        params.push(sd.to_rfc3339());
        param_pos += 1;
    }

    if let Some(ed) = end_date {
        conditions.push(format!("ssi.\"dateTime\" <= ${}", param_pos));
        params.push(ed.to_rfc3339());
        param_pos += 1;
    }

    if let Some(sq) = search_query {
        conditions.push(format!(
            "(ssi.\"fullProductName\" ILIKE ${} OR ssi.\"brandName\" ILIKE ${} OR ssi.\"searchString\" ILIKE ${})",
            param_pos, param_pos + 1, param_pos + 2
        ));
        let search_pattern = format!("%{}%", sq);
        params.push(search_pattern.clone());
        params.push(search_pattern.clone());
        params.push(search_pattern);
        param_pos += 3;
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        r#"
        SELECT
            ssi.id,
            ssi."userName",
            ssi."cityName",
            ssi."dateTime",
            ssi."imageUrl",
            ssi."brandName",
            ssi."fullProductName",
            ssi."oAuthProviderName",
            ssi."targetUserName",
            ssi.quote,
            ssi."voteTypeId",
            ssi."productTypeName",
            ssi."socialShoppingTransactionTypeId",
            ssi.url,
            ssi.rating,
            ssi."searchString",
            dp."amountInclusive" AS amount_inclusive,
            dp."amountExclusive" AS amount_exclusive,
            dp.currency AS currency
        FROM
            "SocialShoppingItem" ssi
        LEFT JOIN
            "DisplayPrice" dp ON ssi.id = dp."socialShoppingItemId"
        WHERE
            {}
        ORDER BY
            ssi."dateTime" DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_clause,
        param_pos,
        param_pos + 1
    );

    let mut query = sqlx::query(&sql);
    for param in params {
        query = query.bind(param);
    }
    query = query.bind(limit);
    query = query.bind(offset);

    let items = query
        .map(|row: PgRow| map_row_to_feed_item(&row))
        .fetch_all(pool)
        .await?;

    items.into_iter().collect()
}

pub async fn get_total_count_with_filters(
    pool: &PgPool,
    transaction_type: Option<i32>,
    brand: Option<String>,
    city: Option<String>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    search_query: Option<String>,
) -> Result<i64, Error> {
    let mut conditions = vec!["1=1".to_string()];
    let mut params: Vec<String> = vec![];
    let mut param_count = 1;

    if let Some(tt) = transaction_type {
        conditions.push(format!("ssi.\"socialShoppingTransactionTypeId\" = ${}", param_count));
        params.push(tt.to_string());
        param_count += 1;
    }

    if let Some(b) = brand {
        conditions.push(format!("ssi.\"brandName\" = ${}", param_count));
        params.push(b);
        param_count += 1;
    }

    if let Some(c) = city {
        conditions.push(format!("ssi.\"cityName\" = ${}", param_count));
        params.push(c);
        param_count += 1;
    }

    if let Some(sd) = start_date {
        conditions.push(format!("ssi.\"dateTime\" >= ${}", param_count));
        params.push(sd.to_rfc3339());
        param_count += 1;
    }

    if let Some(ed) = end_date {
        conditions.push(format!("ssi.\"dateTime\" <= ${}", param_count));
        params.push(ed.to_rfc3339());
        param_count += 1;
    }

    if let Some(sq) = search_query {
        conditions.push(format!(
            "(ssi.\"fullProductName\" ILIKE ${} OR ssi.\"brandName\" ILIKE ${} OR ssi.\"searchString\" ILIKE ${})",
            param_count, param_count + 1, param_count + 2
        ));
        let search_pattern = format!("%{}%", sq);
        params.push(search_pattern.clone());
        params.push(search_pattern.clone());
        params.push(search_pattern);
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        r#"
        SELECT COUNT(*)
        FROM "SocialShoppingItem" ssi
        WHERE {}
        "#,
        where_clause
    );

    let mut query = sqlx::query(&sql);
    for param in params {
        query = query.bind(param);
    }

    let count: i64 = query.fetch_one(pool).await?.get(0);
    Ok(count)
}

pub async fn get_unique_brands(pool: &PgPool) -> Result<Vec<String>, Error> {
    let sql = r#"
        SELECT DISTINCT "brandName"
        FROM "SocialShoppingItem"
        WHERE "brandName" IS NOT NULL
        ORDER BY "brandName"
    "#;

    let brands = sqlx::query(sql)
        .map(|row: PgRow| row.get::<String, _>("brandName"))
        .fetch_all(pool)
        .await?;

    Ok(brands)
}

pub async fn get_unique_cities(pool: &PgPool) -> Result<Vec<String>, Error> {
    let sql = r#"
        SELECT DISTINCT "cityName"
        FROM "SocialShoppingItem"
        WHERE "cityName" IS NOT NULL
        ORDER BY "cityName"
    "#;

    let cities = sqlx::query(sql)
        .map(|row: PgRow| row.get::<String, _>("cityName"))
        .fetch_all(pool)
        .await?;

    Ok(cities)
}

pub async fn get_brands_paginated(
    pool: &PgPool, 
    page: i64, 
    limit: i64, 
    search: Option<String>
) -> Result<(Vec<String>, i64), Error> {
    let offset = (page - 1) * limit;
    
    // Build the query based on whether search term is provided
    let (query_sql, count_sql) = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        (
            format!(
                r#"
                SELECT DISTINCT "brandName"
                FROM "SocialShoppingItem"
                WHERE "brandName" IS NOT NULL
                AND "brandName" ILIKE $1
                ORDER BY "brandName"
                LIMIT $2 OFFSET $3
                "#
            ),
            format!(
                r#"
                SELECT COUNT(DISTINCT "brandName")
                FROM "SocialShoppingItem"
                WHERE "brandName" IS NOT NULL
                AND "brandName" ILIKE $1
                "#
            )
        )
    } else {
        (
            format!(
                r#"
                SELECT DISTINCT "brandName"
                FROM "SocialShoppingItem"
                WHERE "brandName" IS NOT NULL
                ORDER BY "brandName"
                LIMIT $1 OFFSET $2
                "#
            ),
            format!(
                r#"
                SELECT COUNT(DISTINCT "brandName")
                FROM "SocialShoppingItem"
                WHERE "brandName" IS NOT NULL
                "#
            )
        )
    };
    
    // Execute queries in parallel using tokio::join!
    let brands_future = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        sqlx::query(&query_sql)
            .bind(pattern)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| row.get::<String, _>("brandName"))
            .fetch_all(pool)
    } else {
        sqlx::query(&query_sql)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| row.get::<String, _>("brandName"))
            .fetch_all(pool)
    };
    
    let count_future = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        sqlx::query_scalar::<_, i64>(&count_sql)
            .bind(pattern)
            .fetch_one(pool)
    } else {
        sqlx::query_scalar::<_, i64>(&count_sql)
            .fetch_one(pool)
    };
    
    let (brands_result, count_result) = tokio::join!(brands_future, count_future);
    
    match (brands_result, count_result) {
        (Ok(brands), Ok(count)) => Ok((brands, count)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub async fn get_cities_paginated(
    pool: &PgPool, 
    page: i64, 
    limit: i64, 
    search: Option<String>
) -> Result<(Vec<String>, i64), Error> {
    let offset = (page - 1) * limit;
    
    // Build the query based on whether search term is provided
    let (query_sql, count_sql) = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        (
            format!(
                r#"
                SELECT DISTINCT "cityName"
                FROM "SocialShoppingItem"
                WHERE "cityName" IS NOT NULL
                AND "cityName" ILIKE $1
                ORDER BY "cityName"
                LIMIT $2 OFFSET $3
                "#
            ),
            format!(
                r#"
                SELECT COUNT(DISTINCT "cityName")
                FROM "SocialShoppingItem"
                WHERE "cityName" IS NOT NULL
                AND "cityName" ILIKE $1
                "#
            )
        )
    } else {
        (
            format!(
                r#"
                SELECT DISTINCT "cityName"
                FROM "SocialShoppingItem"
                WHERE "cityName" IS NOT NULL
                ORDER BY "cityName"
                LIMIT $1 OFFSET $2
                "#
            ),
            format!(
                r#"
                SELECT COUNT(DISTINCT "cityName")
                FROM "SocialShoppingItem"
                WHERE "cityName" IS NOT NULL
                "#
            )
        )
    };
    
    // Execute queries in parallel using tokio::join!
    let cities_future = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        sqlx::query(&query_sql)
            .bind(pattern)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| row.get::<String, _>("cityName"))
            .fetch_all(pool)
    } else {
        sqlx::query(&query_sql)
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| row.get::<String, _>("cityName"))
            .fetch_all(pool)
    };
    
    let count_future = if let Some(search_term) = &search {
        let pattern = format!("%{}%", search_term);
        sqlx::query_scalar::<_, i64>(&count_sql)
            .bind(pattern)
            .fetch_one(pool)
    } else {
        sqlx::query_scalar::<_, i64>(&count_sql)
            .fetch_one(pool)
    };
    
    let (cities_result, count_result) = tokio::join!(cities_future, count_future);
    
    match (cities_result, count_result) {
        (Ok(cities), Ok(count)) => Ok((cities, count)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}