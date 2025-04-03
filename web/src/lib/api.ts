import type { FeedItem, PaginatedResponse } from '$lib/types';

// Read the backend URL from environment variables
// Fallback to localhost:3133 if not set
const BASE_URL = import.meta.env.VITE_BACKEND_URL || 'http://localhost:3133';

console.log(`Using backend URL: ${BASE_URL}`); // Add this line for debugging

export async function fetchFeedItems(page = 1, limit = 10): Promise<PaginatedResponse<FeedItem>> {
    const url = new URL(`${BASE_URL}/feed`);
    url.searchParams.append('page', page.toString());
    url.searchParams.append('limit', limit.toString());

    try {
        console.log(`Fetching from: ${url.toString()}`); // Add this line for debugging
        const response = await fetch(url.toString());
        if (!response.ok) {
            console.error(`HTTP error response: ${response.status} ${response.statusText}`);
            const errorBody = await response.text();
            console.error(`Error body: ${errorBody}`);
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: PaginatedResponse<FeedItem> = await response.json();
        return data;
    } catch (error) {
        console.error('Failed to fetch feed items:', error);
        // Return a default empty response or re-throw the error
        return { items: [], total: 0, page: 1, limit, totalPages: 0 };
    }
} 