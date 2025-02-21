export interface DisplayPrice {
  amountInclusive: number;
  amountExclusive: number;
  currency: string;
}

export interface Transaction {
  id: string;
  userName: string;
  cityName: string;
  dateTime: string;
  imageUrl: string;
  brandName: string;
  fullProductName: string;
  displayPrice: DisplayPrice;
  url: string;
}