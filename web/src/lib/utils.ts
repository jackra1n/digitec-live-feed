import type { FeedItem } from '$lib/types';

// Define Text Segment Type
export interface TextSegment {
	content: string;
	style: 'normal' | 'user' | 'price' | 'city' | 'link' | 'quote' | 'rating' | 'brand';
	url?: string; // For links
}

// Mapping from ID to display name and template
export const transactionTypeInfo: Record<number, { name: string; template?: string }> = {
    1: { name: 'Rated Product', template: '[0] from [1] rated [2] with [3] stars' },
    2: { name: 'Created Account', template: '[0] from [1] just registered' },
    3: { name: 'Ordered Product', template: '[0] from [1] ordered [2] for [3]' },
    4: { name: 'Picked Up Product', template: '[0] picked up [1] for [2] in store [3]' },
    5: { name: 'Shipped Product', template: '[0] for [1] was sent to [2] from [3]' },
    6: { name: 'Watched Product', template: '[0] from [1] is looking at [2] for [3]' },
    7: { name: 'Searched', template: '[0] from [1] is looking for "[2]"' },
    8: { name: 'Connected Account', template: '[0] from [1] connected account via [2]' },
    9: { name: 'Voted on Rating', template: '[0] found [1]\'s review [2]' },
    10: { name: 'Commented on Rating', template: '[0] commented on [1]\'s rating: [2]' },
    11: { name: 'Asked Question', template: '[0] asked a question about [1]: [2]' },
    12: { name: 'Answered Question', template: '[0] answered [1]\'s question: [2]' },
    13: { name: 'Voted on Question', template: '[0] found [1]\'s question [2]' },
    14: { name: 'Voted on Answer', template: '[0] found [1]\'s answer [2]' },
    15: { name: 'Commented on Answer', template: '[0] commented on [1]\'s answer: [2]' },
    16: { name: 'Started Discussion', template: '[0] started a discussion about [1]: [2]' },
    17: { name: 'Contributed to Discussion', template: '[0] contributed to discussion about [1]: [2]' },
    18: { name: 'Voted on Discussion Post', template: '[0] found [1]\'s post [2]' },
    19: { name: 'Liked Brand', template: '[0] likes [1]' }
};

// Function to generate descriptive text segments based on type and item data
export function getFeedItemDetailsSegments(item: FeedItem): TextSegment[] {
    const info = transactionTypeInfo[item.socialShoppingTransactionTypeId];
    const segments: TextSegment[] = [];

    const add = (content: string, style: TextSegment['style'], url?: string) => {
        if (content?.trim()) {
            segments.push({ content: content.trim(), style, url });
        }
    };

    const city = item.cityName ?? 'somewhere';
    const productName = item.fullProductName?.trim() ?? 'a product';
    const brandName = item.brandName ?? 'a brand';
    const targetUser = item.targetUserName ?? 'someone';
    const quote = item.quote ?? '';
    const ratingValue = item.rating;
    const ratingText = ratingValue ? `${ratingValue}/5` : '';
    const price = item.displayPrice ? `${item.displayPrice.amountInclusive} ${item.displayPrice.currency}` : '';
    const oauthProvider = item.oAuthProviderName ?? 'an external service';
    const search = item.searchString ?? 'something';
    const user = item.userName;

    try {
        switch (item.socialShoppingTransactionTypeId) {
            case 1: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' rated ', 'normal'); add(brandName, 'brand'); add(productName, 'link', item.url); add(' with ', 'normal'); add(ratingText, 'rating'); add(' stars', 'normal'); break;
            case 2: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' just registered', 'normal'); break;
            case 3: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' ordered ', 'normal'); add(brandName, 'brand'); add(productName, 'link', item.url); add(' for ', 'normal'); add(price, 'price'); break;
            case 4: add(user, 'user'); add(' picked up ', 'normal'); add(brandName, 'brand'); add(productName, 'link', item.url); add(' for ', 'normal'); add(price, 'price'); add(' in store ', 'normal'); add(city, 'city'); break;
            case 5: add(brandName, 'brand'); add(productName, 'link', item.url); add(' for ', 'normal'); add(price, 'price'); add(' was sent to ', 'normal'); add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); break;
            case 6: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' is looking at ', 'normal'); add(brandName, 'brand'); add(productName, 'link', item.url); add(' for ', 'normal'); add(price, 'price'); break;
            case 7: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' is looking for ', 'normal'); add(`"${search}"`, 'quote'); break;
            case 8: add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' connected account via ', 'normal'); add(oauthProvider, 'brand'); break;
            case 9: case 13: case 14: case 18: add(user, 'user'); add(' found ', 'normal'); add(targetUser, 'user'); add(`'s ${info?.name.split(' ').pop()?.toLowerCase()} `, 'normal'); add(`"${quote}"`, 'quote', item.url); break;
            case 10: case 15: add(user, 'user'); add(' commented on ', 'normal'); add(targetUser, 'user'); add(`'s ${info?.name.split(' ').pop()?.toLowerCase()}: `, 'normal'); add(`"${quote}"`, 'quote', item.url); break;
            case 11: case 16: case 17: add(user, 'user'); add(` ${info?.name.toLowerCase()} about `, 'normal'); add(brandName, 'brand'); add(productName, 'link', item.url); add(': ', 'normal'); add(`"${quote}"`, 'quote'); break;
            case 12: add(user, 'user'); add(' answered ', 'normal'); add(targetUser, 'user'); add(`'s question: `, 'normal'); add(`"${quote}"`, 'quote', item.url); break;
            case 19: add(user, 'user'); add(' likes ', 'normal'); add(brandName, 'brand', item.url); break;
            default: add(info?.name || `Unknown Action (${item.socialShoppingTransactionTypeId})`, 'normal');
        }
    } catch (err) {
        console.error("Error generating feed segments for item:", item, err);
        segments.push({ content: info?.name || `Unknown Action (${item.socialShoppingTransactionTypeId})`, style: 'normal' }); // Fallback
    }

    return segments;
}

// Format DateTime
export function formatDateTime(isoString: string): string {
    try {
        return new Date(isoString).toLocaleString();
    } catch {
        return 'Invalid Date';
    }
} 