import type { Product, City, SalesData } from '../types';

export const products: Product[] = [
  { id: '1', name: 'Laptop', category: 'Electronics' },
  { id: '2', name: 'Smartphone', category: 'Electronics' },
  { id: '3', name: 'Desk Chair', category: 'Furniture' },
  { id: '4', name: 'Coffee Maker', category: 'Appliances' },
];

export const cities: City[] = [
  { id: '1', name: 'Zürich' },
  { id: '2', name: 'Geneva' },
  { id: '3', name: 'Basel' },
  { id: '4', name: 'Bern' },
  { id: '5', name: 'Lausanne' },
  { id: '6', name: 'Winterthur' },
  { id: '7', name: 'Lucerne' },
  { id: '8', name: 'St. Gallen' },
  { id: '9', name: 'Lugano' },
  { id: '10', name: 'Biel' }
];

export const salesData: SalesData[] = [
  { id: '1', productId: '1', cityId: '1', date: '2024-01-01', amount: 1200, quantity: 2 },
  { id: '2', productId: '2', cityId: '2', date: '2024-01-15', amount: 800, quantity: 1 },
  { id: '3', productId: '3', cityId: '3', date: '2024-02-01', amount: 300, quantity: 3 },
  { id: '4', productId: '4', cityId: '4', date: '2024-02-15', amount: 150, quantity: 2 },
];