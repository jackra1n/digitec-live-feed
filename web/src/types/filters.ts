export interface FilterState {
  cities: string[];
  products: string[];
  brands: string[];
  startDate: string;
  endDate: string;
  priceRange: {
    min: number;
    max: number;
  };
}