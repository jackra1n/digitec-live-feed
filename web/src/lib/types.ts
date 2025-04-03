export interface DisplayPrice {
    amountInclusive: string; // Assuming decimal is serialized as string
    amountExclusive: string;
    currency: string;
}

export interface FeedItem {
    id: number;
    userName: string;
    cityName: string | null;
    dateTime: string; // ISO 8601 string
    imageUrl: string | null;
    brandName: string | null;
    fullProductName: string | null;
    displayPrice: DisplayPrice | null;
    oAuthProviderName: string | null;
    targetUserName: string | null;
    quote: string | null;
    voteTypeId: number | null;
    productTypeName: string | null;
    socialShoppingTransactionTypeId: number;
    url: string;
    rating: number | null;
    searchString: string | null;
}

export interface PaginatedResponse<T> {
    items: T[];
    total: number;
    page: number;
    limit: number;
    totalPages: number;
} 