
const DIGITEC_FETCH_URL = 'https://www.digitec.ch/api/graphql/get-social-shoppings';

const HEADERS = {
  'Accept': '*/*',
  'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36',
  'Content-Type': 'application/json',
  'Origin': 'https://www.digitec.ch'
};

const GRAPHQL_BODY = [{
  operationName: 'GET_SOCIAL_SHOPPINGS',
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
  variables: { 'take': 6, 'latest': null }
}];

const BODY_CONTENT = JSON.stringify(GRAPHQL_BODY);

export const fetchFeedItems = async () => {
  const response = await fetch(DIGITEC_FETCH_URL, {
    method: 'POST',
    headers: HEADERS,
    body: BODY_CONTENT
  })

  const data = await response.json()

  return data[0].data.socialShopping.items;
};