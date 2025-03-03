package main

import (
	"bytes"
	"context"
	"encoding/json"
	"io"
	"log"
	"net/http"
	"os"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
)

const DIGITEC_FETCH_URL = "https://www.digitec.ch/api/graphql/get-social-shoppings"

var HEADERS = map[string]string{
	"Accept":       "*/*",
	"User-Agent":   "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3",
	"Content-Type": "application/json",
	"Origin":       "https://www.digitec.ch",
	"Referer":      "https://www.digitec.ch/",
}

type GraphQLRequest struct {
	OperationName string                 `json:"operationName"`
	Query         string                 `json:"query"`
	Variables     map[string]interface{} `json:"variables"`
}

type LiveFeedEntry struct {
	ID              string `json:"id"`
	UserName        string `json:"userName"`
	CityName        string `json:"cityName"`
	DateTime        string `json:"dateTime"`
	ImageURL        string `json:"imageUrl"`
	BrandName       string `json:"brandName"`
	FullProductName string `json:"fullProductName"`
	DisplayPrice    struct {
		AmountInclusive float64 `json:"amountInclusive"`
		AmountExclusive float64 `json:"amountExclusive"`
		Currency        string  `json:"currency"`
	} `json:"displayPrice"`
	OAuthProviderName               string  `json:"oAuthProviderName"`
	TargetUserName                  string  `json:"targetUserName"`
	Quote                           string  `json:"quote"`
	VoteTypeID                      int     `json:"voteTypeId"`
	ProductTypeName                 string  `json:"productTypeName"`
	SocialShoppingTransactionTypeID int     `json:"socialShoppingTransactionTypeId"`
	URL                             string  `json:"url"`
	Rating                          float64 `json:"rating"`
	SearchString                    string  `json:"searchString"`
}

type GraphQLResponse struct {
	Data struct {
		SocialShopping struct {
			LatestTransactionTimeStamp string          `json:"latestTransactionTimeStamp"`
			Items                      []LiveFeedEntry `json:"items"`
		} `json:"socialShopping"`
	} `json:"data"`
}

type LiveFeedItemsCache struct {
	items   []LiveFeedEntry
	maxSize int
}

func NewLiveFeedItemsCache(size int) *LiveFeedItemsCache {
	return &LiveFeedItemsCache{items: make([]LiveFeedEntry, 0, size), maxSize: size}
}

func (cache *LiveFeedItemsCache) Add(item LiveFeedEntry) {
	if len(cache.items) >= cache.maxSize {
		cache.items = cache.items[1:]
	}
	cache.items = append(cache.items, item)
}

func (cache *LiveFeedItemsCache) GetItems() []LiveFeedEntry {
	return cache.items
}

func FetchItems() []LiveFeedEntry {
	graphqlBody := GraphQLRequest{
		OperationName: "GET_SOCIAL_SHOPPINGS",
		Query: `query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {
        socialShopping(take: $take, latest: $latest) {
          latestTransactionTimeStamp
          items {
            id
            userName
            cityName
            dateTime
            imageUrl
            brandName
            fullProductName
            displayPrice {
              amountInclusive
              amountExclusive
              currency
            }
            oAuthProviderName
            targetUserName
            quote
            voteTypeId
            productTypeName
            socialShoppingTransactionTypeId
            url
            rating
            searchString
          }
        }
      }`,
		Variables: map[string]interface{}{
			"take":   6,
			"latest": nil,
		},
	}

	bodyContent, err := json.Marshal(graphqlBody)
	if err != nil {
		log.Fatal("Error encoding JSON:", err)
	}

	req, err := http.NewRequest("POST", DIGITEC_FETCH_URL, bytes.NewBuffer(bodyContent))
	if err != nil {
		log.Fatal("Error creating request:", err)
	}

	for key, value := range HEADERS {
		req.Header.Set(key, value)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatal("Error making request:", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Fatal("Error reading response:", err)
	}

	var graphqlResponse GraphQLResponse
	err = json.Unmarshal(body, &graphqlResponse)
	if err != nil {
		log.Fatal("Error decoding JSON:", err)
	}

	return graphqlResponse.Data.SocialShopping.Items
}

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

func main() {
	dbpool, err := pgxpool.New(context.Background(), os.Getenv("DATABASE_URL"))
	if err != nil {
		log.Fatal("Error connecting to database:", err)
	}
	defer dbpool.Close()

	var items []LiveFeedEntry
	items = FetchItems()

	// save items to database

}
