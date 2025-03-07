package main

import (
	"bytes"
	"encoding/json"
	"io"
	"log"
	"net/http"
)

const DIGITEC_FETCH_URL = "https://www.digitec.ch/api/graphql/get-social-shoppings"

var HEADERS = map[string]string{
	"Accept":       "*/*",
	"User-Agent":   "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.3",
	"Content-Type": "application/json",
	"Origin":       "https://www.digitec.ch",
	"Referer":      "https://www.digitec.ch/",
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
