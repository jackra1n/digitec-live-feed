
const DIGITEC_FETCH_URL = "https://www.digitec.ch/api/graphql/get-social-shoppings"

const HEADERS = {
  "Accept": "*/*",
  "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36",
  "Content-Type": "application/json",
  "Origin": "https://www.digitec.ch"
}

const GRAPHQL_BODY = [{
  operationName: "GET_SOCIAL_SHOPPINGS",
  query: `query GET_SOCIAL_SHOPPINGS($take: Int, $latest: String) {
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
            salesPrice {
              amountIncl
              amountExcl
              currency
              __typename
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
            __typename
          }
          __typename
        }
      }`,
  variables: { "take": 6, "latest": null }
}]

const BODY_CONTENT = JSON.stringify(GRAPHQL_BODY)


export const load = () => {
  return fetch(DIGITEC_FETCH_URL, {
    method: "POST",
    headers: HEADERS,
    body: BODY_CONTENT
  })
    .then(response => response.json())
    .then(data => {
      return { data };
    })
    .catch(error => {
      console.error(error)
    })
}
