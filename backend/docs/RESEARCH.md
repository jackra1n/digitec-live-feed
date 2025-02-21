digitec live feed website code is dumped in `resources/digitec-live-feed_website.js`

We can send a POST request with some headers to get a response:
```sh
curl 'https://www.digitec.ch/api/graphql/get-social-shoppings' \
  -H 'accept: */*' \
  -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36' \
  -H 'content-type: application/json' \
  -H 'origin: https://www.digitec.ch' \
  --data-raw '[{"operationName":"GET_SOCIAL_SHOPPINGS","variables":{"take":6,"latest":null},"query":"query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {\n  socialShopping(take: $take, latest: $latest) {\n    latestTransactionTimeStamp\n    items {\n      ...SocialShoppingItem\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment SocialShoppingItem on SocialShoppingItem {\n  id\n  userName\n  cityName\n  dateTime\n  imageUrl\n  brandName\n  fullProductName\n  displayPrice {\n    amountInclusive\n    amountExclusive\n    currency\n    __typename\n  }\n  oAuthProviderName\n  targetUserName\n  quote\n  voteTypeId\n  productTypeName\n  socialShoppingTransactionTypeId\n  url\n  rating\n  searchString\n  __typename\n}"}]'
```

Example response:
```json
{
  "data": {
    "socialShopping": {
      "latestTransactionTimeStamp": "1740126603",
      "items": [
        {
          "id": "708315505",
          "userName": "E.",
          "cityName": "Sutz",
          "dateTime": "2025-02-21T08:30:02.2438000Z",
          "imageUrl": "//static.digitecgalaxus.ch/productimages/5/2/4/9/0/8/6/1/8/8/1/8/6/9/5/6/5/1/9/dc2cfd81-0271-454a-b674-68a9dc9e8c8b_cropped.jpg?fit=inside%7C160:128",
          "brandName": "Reer",
          "fullProductName": " Mummy Me",
          "displayPrice": {
            "amountInclusive": 7.9,
            "amountExclusive": 7.31,
            "currency": "CHF"
          },
          "oAuthProviderName": null,
          "targetUserName": null,
          "quote": null,
          "voteTypeId": null,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 5,
          "url": "/de/s14/product/reer-mummy-me-nachtlicht-5683079",
          "rating": null,
          "searchString": null
        },
        {
          "id": "27631422",
          "userName": "Digmesa",
          "cityName": null,
          "dateTime": "2025-02-21T08:29:51.1414000Z",
          "imageUrl": "//static.digitecgalaxus.ch/productimages/5/5/0/4/3/4/6/0/1/6/2/4/4/9/0/2/6/1/6/16df87ff-262b-46a2-a54f-fca070b28444_cropped.jpg?fit=inside%7C160:128",
          "brandName": "Philips Hue",
          "fullProductName": " Tap dial switch",
          "displayPrice": {
            "amountInclusive": 34.7,
            "amountExclusive": 32.1,
            "currency": "CHF"
          },
          "oAuthProviderName": null,
          "targetUserName": "michaeljugl81",
          "quote": "Vielseitig für einen Schalter",
          "voteTypeId": 1,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 9,
          "url": "/de/productrating/4564075",
          "rating": null,
          "searchString": null
        },
        {
          "id": "749730504",
          "userName": "A.",
          "cityName": "Echandens",
          "dateTime": "2025-02-21T08:29:27.9726000Z",
          "imageUrl": "//static.digitecgalaxus.ch/productimages/2/2/5/8/4/7/6/1/5/7/2/2/0/3/3/2/8/8/7/22d995f8-c240-4329-ac8c-9c5fb20226c3_cropped.jpg?fit=inside%7C160:128",
          "brandName": "Joseph Joseph",
          "fullProductName": " Tota",
          "displayPrice": {
            "amountInclusive": 95.9,
            "amountExclusive": 88.71,
            "currency": "CHF"
          },
          "oAuthProviderName": null,
          "targetUserName": null,
          "quote": null,
          "voteTypeId": null,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 3,
          "url": "/de/s14/product/joseph-joseph-tota-60-l-waeschekorb-24534849",
          "rating": null,
          "searchString": null
        },
        {
          "id": "23012812",
          "userName": "D.",
          "cityName": "Bellinzona",
          "dateTime": "2025-02-21T08:29:16.4818000Z",
          "imageUrl": null,
          "brandName": null,
          "fullProductName": null,
          "displayPrice": null,
          "oAuthProviderName": null,
          "targetUserName": null,
          "quote": null,
          "voteTypeId": null,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 2,
          "url": "/de/",
          "rating": null,
          "searchString": null
        },
        {
          "id": "2011414820",
          "userName": "G.",
          "cityName": "Zürich",
          "dateTime": "2025-02-21T08:29:13.5852000Z",
          "imageUrl": "//static.digitecgalaxus.ch/productimages/2/9/0/2/9/0/3/8/6/8/6/4/0/6/7/5/2/7/7/ef1800b1-ae16-4f14-943a-c0a43c4da7d3.png?fit=inside%7C160:128",
          "brandName": "dm Balea",
          "fullProductName": " Haarentfärbungscreme",
          "displayPrice": {
            "amountInclusive": 9.9,
            "amountExclusive": 9.16,
            "currency": "CHF"
          },
          "oAuthProviderName": null,
          "targetUserName": null,
          "quote": null,
          "voteTypeId": null,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 4,
          "url": "/de/s6/product/dm-balea-haarentfaerbungscreme-wachs-enthaarungscreme-20834692",
          "rating": null,
          "searchString": null
        },
        {
          "id": "1512285748",
          "userName": "basic_chino",
          "cityName": "Buch am Irchel",
          "dateTime": "2025-02-21T08:29:09.2428000Z",
          "imageUrl": "//static.digitecgalaxus.ch/productimages/4/4/9/6/0/7/9/1/8/8/9/6/1/7/3/1/1/2/5/9c2db495-bce3-4b85-99a9-df7e0b649146_cropped.jpg?fit=inside%7C160:128",
          "brandName": "Nintendo",
          "fullProductName": " Super Mario Party Jamboree",
          "displayPrice": {
            "amountInclusive": 47.7,
            "amountExclusive": 44.13,
            "currency": "CHF"
          },
          "oAuthProviderName": null,
          "targetUserName": null,
          "quote": "Spass für die ganze Familie",
          "voteTypeId": null,
          "productTypeName": null,
          "socialShoppingTransactionTypeId": 1,
          "url": "/de/productrating/8557830",
          "rating": 5,
          "searchString": null
        }
      ]
    }
  }
}
```

Based on that we know the different types of live feed activities that are defined with `socialShoppingTransactionTypeId`:

| Key                         | ID | Description                                                                        |
|-----------------------------|----|------------------------------------------------------------------------------------|
| RATING                      | 1  | A user rates a product.                                                            |
| USERCREATION                | 2  | A user creates an account.                                                         |
| ORDEREDPRODUCT              | 3  | A user orders a product.                                                           |
| PICKEDUPPRODUCT             | 4  | A user picks up a product in-store.                                                |
| SHIPPEDPRODUCT              | 5  | A product is shipped to a user.                                                    |
| WATCHEDPRODUCT              | 6  | A user views or "watches" a product.                                               |
| SEARCH                      | 7  | A user performs a search query.                                                    |
| OAUTHUSERCONNECTED          | 8  | A user connects their account using an OAuth provider (e.g., Google, Facebook).    |
| USERVOTESONRATING           | 9  | A user votes on another user's product rating (e.g., "helpful" or "not helpful").  |
| USERCOMMENTSONRATING        | 10 | A user comments on another user's product rating.                                  |
| USERASKSQUESTION            | 11 | A user asks a question (e.g., about a product or a topic).                         |
| USERANSWERSQUESTION         | 12 | A user answers a question.                                                         |
| USERVOTESONQUESTION         | 13 | A user votes on another user's question.                                           |
| USERVOTESONANSWER           | 14 | A user votes on another user's answer.                                             |
| USERCOMMENTSONANSWER        | 15 | A user comments on another user's answer.                                          |
| USERSTARTSDISCUSSION        | 16 | A user starts a discussion (e.g., about a product or topic).                       |
| USERCONTRIBUTESTODISCUSSION | 17 | A user contributes to an ongoing discussion (e.g., replying to posts or comments). |
