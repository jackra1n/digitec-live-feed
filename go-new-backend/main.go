package main

import (
	"context"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
)

func main() {
	pool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		panic("Unable to connect to database: " + err.Error())
	}
	defer pool.Close()

	cache := NewFeedItemCache(50)

	fetchedItems := FetchItems()
	newItems := cache.ProcessAndFilterNew(fetchedItems)

	saveItemsToDB(pool, newItems)
}
