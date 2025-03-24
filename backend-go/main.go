package main

import (
	"context"
	"flag"
	"log"
	"os"

	"github.com/jackc/pgx/v5/pgxpool"
)

func main() {
	flag.Parse()

	dbpool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatal("Error connecting to database:", err)
	}
	defer dbpool.Close()

	itemsCache := *NewLiveFeedItemsCache(50)

	fetchedItems := FetchItems()
	newItems := filterNewItems(fetchedItems, itemsCache.items)

	for _, newItem := range newItems {
		itemsCache.Add(newItem)
	}

	saveItemsToDB(dbpool, newItems)
}
