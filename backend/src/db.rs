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