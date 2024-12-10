
const DIGITEC_FETCH_URL = 'https://www.digitec.ch/api/graphql/get-social-shoppings';

const HEADERS = {
	'Accept': '*/*',
	'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1',
	'Content-Type': 'application/json',
	'Origin': 'https://www.digitec.ch',
	'Referer': 'https://www.digitec.ch/'
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

let lastFetchedItemIds = [];


export const fetchFeedItems = async () => {
	const response = await fetch(DIGITEC_FETCH_URL, {
		method: 'POST',
		headers: HEADERS,
		body: BODY_CONTENT
	});

	if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to fetch feed items: ${response.status} ${response.statusText}. Response: ${errorText}`);
  }

	const data = await response.json();
	const currentItems = data[0].data.socialShopping.items;

	const currentItemIds = currentItems.map(generateItemIdentifier);
	const newIds = currentItemIds.filter((id) => !lastFetchedItemIds.includes(id));

	const newItems = currentItems.filter((item) => {
		const itemIdentifier = generateItemIdentifier(item);
		return newIds.includes(itemIdentifier);
	});

	lastFetchedItemIds = [...newIds, ...lastFetchedItemIds].slice(0, 50);
	return newItems;
};


function generateItemIdentifier(item) {
	return `${item.userName}_${item.dateTime}_${item.socialShoppingTransactionTypeId}_${item.url}`;
}