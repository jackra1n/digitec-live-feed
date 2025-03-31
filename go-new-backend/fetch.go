package main

import (
	"bytes"
	"encoding/json"
	"io"
	"log"
	"net/http"
	"strings"
)

const digitecFetchUrl = "https://www.digitec.ch/api/graphql/get-social-shoppings"
const userAgent = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"

var headers = map[string]string{
	"accept":         "*/*",
	"user-agent":     userAgent,
	"content-type":   "application/json",
	"origin":         "https://www.digitec.ch",
	"referer":        "https://www.digitec.ch/en/daily-deal",
	"sec-fetch-mode": "cors",
}

var graphqlBody = GraphQLRequest{
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
		"take":   10,
		"latest": nil,
	},
}

func FetchItems() []FeedItem {
	bodyContent, err := json.Marshal(graphqlBody)
	if err != nil {
		log.Fatal("Error encoding JSON:", err)
	}

	req, err := http.NewRequest("POST", digitecFetchUrl, bytes.NewBuffer(bodyContent))
	if err != nil {
		log.Fatal("Error creating request:", err)
	}

	for key, value := range headers {
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

	items := graphqlResponse.Data.SocialShopping.Items
	for i := range items {
		items[i].FullProductName = strings.TrimSpace(items[i].FullProductName)
	}

	return items
}
