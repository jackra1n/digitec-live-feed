use crate::models::{
    api_input::ApiFeedItem,
    api_output::{DisplayPriceResponse, FeedItemResponse},
    database::NameIdPair,
    filters::FeedItemFilters,
};
use rust_decimal::Decimal;
use sqlx::{postgres::{PgRow, PgQueryResult}, Error, PgPool, QueryBuilder, Row};
use std::collections::{HashMap, HashSet};

pub async fn insert_feed_items_batch(pool: &PgPool, items: &[ApiFeedItem]) -> Result<(), Error> {
    if items.is_empty() {
        return Ok(());
    }

    let mut unique_brands = HashSet::new();
    let mut unique_product_types = HashSet::new();
    let mut unique_cities = HashSet::new();

    for item in items {
        if let Some(brand) = &item.brand_name {
            let trimmed = brand.trim();
            if !trimmed.is_empty() {
                unique_brands.insert(trimmed.to_string());
            }
        }
        if let Some(ptype) = &item.product_type_name {
            let trimmed = ptype.trim();
            if !trimmed.is_empty() {
                unique_product_types.insert(trimmed.to_string());
            }
        }
        if let Some(city) = &item.city_name {
            let trimmed = city.trim();
            if !trimmed.is_empty() {
                trace!(
                    "Item {}: Adding city to unique set: '{}' (original: '{}')",
                    item.id,
                    trimmed,
                    city
                );
                unique_cities.insert(trimmed.to_string());
            } else {
                trace!(
                    "Item {}: Skipping empty/whitespace city: '{}'",
                    item.id,
                    city
                );
            }
        } else {
            trace!("Item {}: City is None", item.id);
        }
    }

    let mut tx = pool.begin().await?;

    let brand_map: HashMap<String, i32> = if !unique_brands.is_empty() {
        let brand_names: Vec<String> = unique_brands.into_iter().collect();
        debug!("Upserting brands: {:?}", brand_names);
        let rows = sqlx::query_as!(
            NameIdPair,
            r#"
            WITH input_rows (name) AS ( SELECT * FROM UNNEST($1::text[]) ),
            inserted AS (
                INSERT INTO "Brand" (name) SELECT name FROM input_rows
                ON CONFLICT (name) DO NOTHING RETURNING id, name
            )
            SELECT id as "id!", name as "name!" -- Add override for name
            FROM inserted
            UNION ALL
            SELECT b.id as "id!", b.name as "name!" -- Add override for name
            FROM "Brand" b JOIN input_rows ir ON b.name = ir.name;
            "#,
            &brand_names
        )
        .fetch_all(&mut *tx)
        .await?;
        rows.into_iter().map(|pair| (pair.name, pair.id)).collect()
    } else {
        HashMap::new()
    };

    let product_type_map: HashMap<String, i32> = if !unique_product_types.is_empty() {
        let ptype_names: Vec<String> = unique_product_types.into_iter().collect();
        log::debug!("Upserting product types: {:?}", ptype_names);
        let rows = sqlx::query_as!(
            NameIdPair,
            r#"
            WITH input_rows (name) AS ( SELECT * FROM UNNEST($1::text[]) ),
            inserted AS (
                INSERT INTO "ProductType" (name) SELECT name FROM input_rows
                ON CONFLICT (name) DO NOTHING RETURNING id, name
            )
            SELECT id as "id!", name as "name!" -- Add override for name
            FROM inserted
            UNION ALL
            SELECT pt.id as "id!", pt.name as "name!" -- Add override for name
            FROM "ProductType" pt JOIN input_rows ir ON pt.name = ir.name;
            "#,
            &ptype_names
        )
        .fetch_all(&mut *tx)
        .await?;
        rows.into_iter().map(|pair| (pair.name, pair.id)).collect()
    } else {
        HashMap::new()
    };

    if !unique_cities.is_empty() {
        let city_names: Vec<String> = unique_cities.iter().cloned().collect();
        log::debug!("Upserting raw cities: {:?}", city_names);
        sqlx::query!(
            r#"
            INSERT INTO "RawCityNameMap" ("rawInput", "mappingStatus")
            SELECT name, 'unprocessed' FROM UNNEST($1::text[]) as name
            ON CONFLICT ("rawInput") DO NOTHING;
            "#,
            &city_names
        )
        .execute(&mut *tx)
        .await?;
        log::debug!("Finished upserting raw cities.");
    } else {
        log::debug!("No unique cities to upsert.");
    }

    for item in items {
        let brand_id = item
            .brand_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .and_then(|name| brand_map.get(name));

        let product_type_id = item
            .product_type_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .and_then(|name| product_type_map.get(name));

        let city_name_to_insert = item
            .city_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        let full_product_name_trimmed = item
            .full_product_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        log::debug!(
            "Attempting insert for item ID {}: Raw City='{:?}', City to Insert='{:?}', BrandID='{:?}', PTypeID='{:?}'",
            item.id,
            item.city_name,
            city_name_to_insert,
            brand_id,
            product_type_id
        );

        let item_insert_result: Result<PgQueryResult, sqlx::Error> = sqlx::query!(
            r#"
            INSERT INTO "SocialShoppingItem" (
                id, "userName", "cityName", "dateTime", "imageUrl", "brandId",
                "fullProductName", "oAuthProviderName", "targetUserName", "quote",
                "voteTypeId", "productTypeId", "socialShoppingTransactionTypeId",
                "url", "rating", "searchString"
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            ON CONFLICT ("userName", "dateTime", "socialShoppingTransactionTypeId", "url") DO NOTHING
            "#,
            item.id as i32,
            item.user_name,
            city_name_to_insert,
            item.date_time,
            item.image_url,
            brand_id.copied(),
            full_product_name_trimmed,
            item.o_auth_provider_name,
            item.target_user_name,
            item.quote,
            item.vote_type_id,
            product_type_id.copied(),
            item.social_shopping_transaction_type_id,
            item.url,
            item.rating,
            item.search_string
        )
        .execute(&mut *tx)
        .await;

        match item_insert_result {
            Ok(query_result) => {
                let inserted_item = query_result.rows_affected() > 0;

                if inserted_item {
                    log::trace!("Successfully inserted or found item ID {}", item.id);
                } else {
                    log::trace!("Skipped inserting duplicate item ID {} based on unique constraint", item.id);
                }

                if inserted_item && item.display_price.is_some() {
                    if let Some(price) = &item.display_price {
                        log::trace!("Attempting to insert price for newly inserted item ID {}", item.id);
                        let price_insert_result = sqlx::query!(
                            r#"
                            INSERT INTO "DisplayPrice" ( "socialShoppingItemId", "amountInclusive", "amountExclusive", "currency" )
                            VALUES ( $1, $2, $3, $4 )
                            -- Assuming you only want one price per item, ON CONFLICT is good.
                            ON CONFLICT ("socialShoppingItemId") DO NOTHING
                            "#,
                            item.id as i32,
                            price.amount_inclusive,
                            price.amount_exclusive,
                            price.currency
                        )
                        .execute(&mut *tx)
                        .await;

                        if let Err(e) = price_insert_result {
                            log::error!(
                                "Error inserting price for NEWLY INSERTED item ID {} within batch transaction: {}. Rolling back.",
                                item.id, e
                            );
                            return Err(e);
                        }
                    }
                } else if !inserted_item && item.display_price.is_some() {
                     log::trace!("Skipped price insert for item ID {} because item insert was skipped.", item.id);
                }

            }
            Err(e) => {
                log::error!(
                    "Error during SocialShoppingItem INSERT for item ID {}: {}. Rolling back.",
                    item.id, e
                );
                 if let Some(db_err) = e.as_database_error() {
                     log::error!(
                         "DB Error details: Constraint={:?}, Code={:?}",
                         db_err.constraint(), db_err.code()
                     );
                 }
                return Err(e);
            }
        }
    }

    tx.commit().await?;
    log::debug!(
        "Successfully committed batch insert of {} items.",
        items.len()
    );
    Ok(())
}

fn map_row_to_feed_item_response(row: &PgRow) -> Result<FeedItemResponse, sqlx::Error> {
    let amount_inclusive: Option<Decimal> = row.try_get("price_amount_inclusive")?;
    let amount_exclusive: Option<Decimal> = row.try_get("price_amount_exclusive")?;
    let currency: Option<String> = row.try_get("price_currency")?;

    let display_price = match (amount_inclusive, amount_exclusive, currency) {
        (Some(ai), Some(ae), Some(c)) => Some(DisplayPriceResponse {
            amount_inclusive: ai,
            amount_exclusive: ae,
            currency: c,
        }),
        _ => None,
    };

    Ok(FeedItemResponse {
        id: row.try_get("ssi_id")?,
        user_name: row.try_get("ssi_userName")?,
        city_name: row.try_get("cc_canonicalName")?,
        canton: row.try_get("cc_canton")?,
        country_code: row.try_get("cc_countryCode")?,
        date_time: row.try_get("ssi_dateTime")?,
        image_url: row.try_get("ssi_imageUrl")?,
        brand_name: row.try_get("b_name")?,
        full_product_name: row.try_get("ssi_fullProductName")?,
        o_auth_provider_name: row.try_get("ssi_oAuthProviderName")?,
        target_user_name: row.try_get("ssi_targetUserName")?,
        quote: row.try_get("ssi_quote")?,
        vote_type_id: row.try_get("ssi_voteTypeId")?,
        product_type_name: row.try_get("pt_name")?,
        social_shopping_transaction_type_id: row.try_get("ssi_socialShoppingTransactionTypeId")?,
        url: row.try_get("ssi_url")?,
        rating: row.try_get("ssi_rating")?,
        search_string: row.try_get("ssi_searchString")?,
        display_price,
    })
}

pub async fn get_feed_items_with_filters(
    pool: &PgPool,
    filters: &FeedItemFilters,
    limit: i64,
    offset: i64,
) -> Result<Vec<FeedItemResponse>, Error> {
    let mut builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
        r#"
        SELECT
            ssi.id as ssi_id,
            ssi."userName" as "ssi_userName",
            ssi."cityName" as "ssi_cityName_raw", -- Keep raw if needed, maybe for debugging
            ssi."dateTime" as "ssi_dateTime",
            ssi."imageUrl" as "ssi_imageUrl",
            ssi."fullProductName" as "ssi_fullProductName",
            ssi."oAuthProviderName" as "ssi_oAuthProviderName",
            ssi."targetUserName" as "ssi_targetUserName",
            ssi.quote as ssi_quote,
            ssi."voteTypeId" as "ssi_voteTypeId",
            ssi."socialShoppingTransactionTypeId" as "ssi_socialShoppingTransactionTypeId",
            ssi.url as ssi_url,
            ssi.rating as ssi_rating,
            ssi."searchString" as "ssi_searchString",
            dp."amountInclusive" AS price_amount_inclusive,
            dp."amountExclusive" AS price_amount_exclusive,
            dp.currency AS price_currency,
            b.name as b_name,                 -- Brand name
            pt.name as pt_name,               -- Product Type name
            cc."canonicalName" as cc_canonicalName, -- Canonical City name
            cc.canton as cc_canton,
            cc."countryCode" as cc_countryCode
        FROM
            "SocialShoppingItem" ssi
        LEFT JOIN "DisplayPrice" dp ON ssi.id = dp."socialShoppingItemId"
        LEFT JOIN "Brand" b ON ssi."brandId" = b.id
        LEFT JOIN "ProductType" pt ON ssi."productTypeId" = pt.id
        LEFT JOIN "RawCityNameMap" rcm ON ssi."cityName" = rcm."rawInput"
        LEFT JOIN "CanonicalCity" cc ON rcm."canonicalCityId" = cc.id
        WHERE 1=1
        "#,
    );

    if let Some(tt) = filters.transaction_type {
        builder.push(" AND ssi.\"socialShoppingTransactionTypeId\" = ");
        builder.push_bind(tt);
    }

    if let Some(b_name) = &filters.brand_name {
        let trimmed_b_name = b_name.trim().to_string();
        if !trimmed_b_name.is_empty() {
            builder.push(" AND b.name = ");
            builder.push_bind(trimmed_b_name);
        }
    }

    if let Some(c_name) = &filters.city_name {
        let trimmed_c_name = c_name.trim().to_string();
        if !trimmed_c_name.is_empty() {
            builder.push(" AND cc.\"canonicalName\" = ");
            builder.push_bind(trimmed_c_name);
            builder.push(" AND rcm.\"mappingStatus\" = 'mapped' ");
        }
    }

    if let Some(sd) = filters.start_date {
        builder.push(" AND ssi.\"dateTime\" >= ");
        builder.push_bind(sd);
    }

    if let Some(ed) = filters.end_date {
        builder.push(" AND ssi.\"dateTime\" <= ");
        builder.push_bind(ed);
    }

    if let Some(sq) = &filters.search {
        let search_pattern = format!("%{}%", sq.trim());
        if !search_pattern.is_empty() && search_pattern != "%%" {
            builder.push(
                r#" AND (
                ssi."fullProductName" ILIKE "#,
            );
            builder.push_bind(search_pattern.clone());
            builder.push(r#" OR b.name ILIKE "#);
            builder.push_bind(search_pattern.clone());
            builder.push(r#" OR pt.name ILIKE "#);
            builder.push_bind(search_pattern.clone());
            builder.push(r#" OR ssi."searchString" ILIKE "#);
            builder.push_bind(search_pattern.clone());
            builder.push(r#" OR ssi."userName" ILIKE "#);
            builder.push_bind(search_pattern);
            builder.push(r#" ) "#);
        }
    }

    builder.push(r#" ORDER BY ssi."dateTime" DESC "#);
    builder.push(" LIMIT ");
    builder.push_bind(limit);
    builder.push(" OFFSET ");
    builder.push_bind(offset);

    let query = builder.build();
    let items = query
        .map(|row: PgRow| map_row_to_feed_item_response(&row))
        .fetch_all(pool)
        .await?;

    items.into_iter().collect::<Result<Vec<_>, _>>()
}

pub async fn get_total_count_with_filters(
    pool: &PgPool,
    filters: &FeedItemFilters,
) -> Result<i64, Error> {
    let mut needs_brand_join = filters.brand_name.is_some();
    let mut needs_ptype_join = false;
    let needs_city_join = filters.city_name.is_some();

    if let Some(sq) = &filters.search {
        let search_pattern = format!("%{}%", sq.trim());
        if !search_pattern.is_empty() && search_pattern != "%%" {
            needs_brand_join = true;
            needs_ptype_join = true;
        }
    }

    let mut builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("SELECT COUNT(ssi.id) FROM \"SocialShoppingItem\" ssi ");

    if needs_brand_join {
        builder.push(" LEFT JOIN \"Brand\" b ON ssi.\"brandId\" = b.id ");
    }
    if needs_ptype_join {
        builder.push(" LEFT JOIN \"ProductType\" pt ON ssi.\"productTypeId\" = pt.id ");
    }
    if needs_city_join {
        builder.push(" LEFT JOIN \"RawCityNameMap\" rcm ON ssi.\"cityName\" = rcm.\"rawInput\" ");
        builder.push(" LEFT JOIN \"CanonicalCity\" cc ON rcm.\"canonicalCityId\" = cc.id ");
    }

    builder.push(" WHERE 1=1 ");

    if let Some(tt) = filters.transaction_type {
        builder.push(" AND ssi.\"socialShoppingTransactionTypeId\" = ");
        builder.push_bind(tt);
    }

    if let Some(b_name) = &filters.brand_name {
        let trimmed_b_name = b_name.trim().to_string();
        if !trimmed_b_name.is_empty() {
            builder.push(" AND b.name = ");
            builder.push_bind(trimmed_b_name);
        }
    }

    if let Some(c_name) = &filters.city_name {
        let trimmed_c_name = c_name.trim().to_string();
        if !trimmed_c_name.is_empty() {
            builder.push(" AND cc.\"canonicalName\" = ");
            builder.push_bind(trimmed_c_name);
            builder.push(" AND rcm.\"mappingStatus\" = 'mapped' ");
        }
    }

    if let Some(sd) = filters.start_date {
        builder.push(" AND ssi.\"dateTime\" >= ");
        builder.push_bind(sd);
    }

    if let Some(ed) = filters.end_date {
        builder.push(" AND ssi.\"dateTime\" <= ");
        builder.push_bind(ed);
    }

    if let Some(sq) = &filters.search {
        let search_pattern = format!("%{}%", sq.trim());
        if !search_pattern.is_empty() && search_pattern != "%%" {
            builder.push(
                r#" AND (
                ssi."fullProductName" ILIKE "#,
            );
            builder.push_bind(search_pattern.clone());
            if needs_brand_join {
                builder.push(r#" OR b.name ILIKE "#);
                builder.push_bind(search_pattern.clone());
            }
            if needs_ptype_join {
                builder.push(r#" OR pt.name ILIKE "#);
                builder.push_bind(search_pattern.clone());
            }
            builder.push(r#" OR ssi."searchString" ILIKE "#);
            builder.push_bind(search_pattern.clone());
            builder.push(r#" OR ssi."userName" ILIKE "#);
            builder.push_bind(search_pattern);
            builder.push(r#" ) "#);
        }
    }

    let query = builder.build_query_scalar();
    let count: i64 = query.fetch_one(pool).await?;

    Ok(count)
}

pub async fn get_brands_paginated(
    pool: &PgPool,
    page: i64,
    limit: i64,
    search: Option<String>,
) -> Result<(Vec<String>, i64), Error> {
    let offset = (page - 1) * limit;

    let mut query_builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new(r#"SELECT name FROM "Brand" WHERE 1=1 "#);
    let mut count_builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new(r#"SELECT COUNT(id) FROM "Brand" WHERE 1=1 "#);

    if let Some(term) = &search {
        let pattern = format!("%{}%", term.trim());
        if !pattern.is_empty() && pattern != "%%" {
            let condition = " AND name ILIKE ";
            query_builder.push(condition);
            query_builder.push_bind(pattern.clone());
            count_builder.push(condition);
            count_builder.push_bind(pattern);
        }
    }

    query_builder.push(" ORDER BY name ");
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let brands_query = query_builder.build_query_scalar();
    let count_query = count_builder.build_query_scalar();

    let (brands_result, count_result) =
        tokio::join!(brands_query.fetch_all(pool), count_query.fetch_one(pool));

    let brands = brands_result?;
    let count = count_result?;

    Ok((brands, count))
}

pub async fn get_cities_paginated(
    pool: &PgPool,
    page: i64,
    limit: i64,
    search: Option<String>,
) -> Result<(Vec<String>, i64), Error> {
    let offset = (page - 1) * limit;

    let base_condition = r#" "isSwiss" = TRUE "#;

    let mut query_builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new(r#"SELECT "canonicalName" FROM "CanonicalCity" WHERE "#);
    query_builder.push(base_condition);

    let mut count_builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new(r#"SELECT COUNT(id) FROM "CanonicalCity" WHERE "#);
    count_builder.push(base_condition);

    if let Some(term) = &search {
        let pattern = format!("%{}%", term.trim());
        if !pattern.is_empty() && pattern != "%%" {
            let condition = r#" AND "canonicalName" ILIKE "#;
            query_builder.push(condition);
            query_builder.push_bind(pattern.clone());
            count_builder.push(condition);
            count_builder.push_bind(pattern);
        }
    }

    query_builder.push(r#" ORDER BY "canonicalName" "#);
    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    let cities_query = query_builder.build_query_scalar();
    let count_query = count_builder.build_query_scalar();

    let (cities_result, count_result) =
        tokio::join!(cities_query.fetch_all(pool), count_query.fetch_one(pool));

    let cities = cities_result?;
    let count = count_result?;

    Ok((cities, count))
}
