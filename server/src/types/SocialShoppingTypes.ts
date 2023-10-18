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

export interface DisplayPrice {
    amountInclusive: number;
    amountExclusive: number;
    currency: string;
}

export interface DigitecSocialShoppingItem {
    id: string;
    displayPrice: DisplayPrice | null;
    dateTime: string;
}
