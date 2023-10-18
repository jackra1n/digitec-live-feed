export enum SocialShoppingType {
    ProductRating = 1,
    Registration = 2,
    OnlineOrder = 3,
    ShopCollection = 4,
    ProductSent = 5,
    ReviewRating = 9,
    AskingQuestion = 11,
    AnsweringQuestion = 12,
    CommentOnAnswer = 15,
    Like = 19
}

export interface DigitecDisplayPrice {
    amountInclusive: number;
    amountExclusive: number;
    currency: string;
}

export interface DigitecSocialShoppingItem {
    id: string;
    userName: string;
    cityName: string | null;
    dateTime: string;
    imageUrl: string | null;
    brandName: string | null;
    fullProductName: string | null;
    displayPrice: DigitecDisplayPrice | null;
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
