package main

type GraphQLRequest struct {
	OperationName string                 `json:"operationName"`
	Query         string                 `json:"query"`
	Variables     map[string]interface{} `json:"variables"`
}

type FeedItem struct {
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
			LatestTransactionTimeStamp string     `json:"latestTransactionTimeStamp"`
			Items                      []FeedItem `json:"items"`
		} `json:"socialShopping"`
	} `json:"data"`
}
