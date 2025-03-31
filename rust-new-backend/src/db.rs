use sqlx::{PgPool, Error};
use crate::types::FeedItem;


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
            item.rating.map(|r| r.round() as i32),
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
                price.amount_inclusive as f64,
                price.amount_exclusive as f64,
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