package main

import (
	"context"
	"log"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

func saveItemsToDB(pool *pgxpool.Pool, items []LiveFeedEntry) {
	conn, err := pool.Acquire(context.Background())
	if err != nil {
		log.Fatal("Error acquiring connection from pool:", err)
	}
	defer conn.Release()

	batch := &pgx.Batch{}

	for _, item := range items {
		batch.Queue(`
            INSERT INTO "SocialShoppingItem" (
                "id", "userName", "cityName", "dateTime", "imageUrl", "brandName", "fullProductName",
                "oAuthProviderName", "targetUserName", "quote", "voteTypeId", "productTypeName",
                "socialShoppingTransactionTypeId", "url", "rating", "searchString"
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            ) ON CONFLICT ("id") DO NOTHING`,
			item.ID, item.UserName, item.CityName, item.DateTime, item.ImageURL, item.BrandName, item.FullProductName,
			item.OAuthProviderName, item.TargetUserName, item.Quote, item.VoteTypeID, item.ProductTypeName,
			item.SocialShoppingTransactionTypeID, item.URL, item.Rating, item.SearchString,
		)

		batch.Queue(`
            INSERT INTO "DisplayPrice" (
                "socialShoppingItemId", "amountInclusive", "amountExclusive", "currency"
            ) VALUES (
                $1, $2, $3, $4
            ) ON CONFLICT ("socialShoppingItemId") DO NOTHING`,
			item.ID, item.DisplayPrice.AmountInclusive, item.DisplayPrice.AmountExclusive, item.DisplayPrice.Currency,
		)
	}

	br := conn.SendBatch(context.Background(), batch)
	defer br.Close()

	if err := br.Close(); err != nil {
		log.Printf("Error executing batch: %v", err)
	}
}
