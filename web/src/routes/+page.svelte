<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchFeedItems } from '$lib/api';
	import type { FeedItem, PaginatedResponse } from '$lib/types';
	import { writable } from 'svelte/store';

	// Define Text Segment Type HERE
	interface TextSegment {
		content: string;
		style: 'normal' | 'user' | 'price' | 'city' | 'link' | 'quote' | 'rating' | 'brand';
		url?: string; // For links
	}

	const feedData = writable<PaginatedResponse<FeedItem> | null>(null);
	const currentPage = writable(1);
	const itemsPerPage = 15; // Or make this configurable
	let isLoading = writable(true);
	let error = writable<string | null>(null);

	// Updated mapping based on provided source code
	const transactionTypeInfo: Record<number, { name: string; template?: string }> = {
		1: { name: 'Rated Product', template: '[0] from [1] rated [2] with [3] stars' },
		2: { name: 'Created Account', template: '[0] from [1] just registered' },
		3: { name: 'Ordered Product', template: '[0] from [1] ordered [2] for [3]' },
		4: { name: 'Picked Up Product', template: '[0] picked up [1] for [2] in store [3]' },
		5: { name: 'Shipped Product', template: '[0] for [1] was sent to [2] from [3]' },
		6: { name: 'Watched Product', template: '[0] from [1] is looking at [2] for [3]' },
		7: { name: 'Searched', template: '[0] from [1] is looking for \"[2]\"' }, // Assuming searchString holds query
		8: { name: 'Connected Account', template: '[0] from [1] connected account via [2]' },
		9: { name: 'Voted on Rating', template: '[0] found [1]\'s review [2]' }, // [2] = quote (e.g., useful/not useful)
		10: { name: 'Commented on Rating', template: '[0] commented on [1]\'s rating: [2]' },
		11: { name: 'Asked Question', template: '[0] asked a question about [1]: [2]' },
		12: { name: 'Answered Question', template: '[0] answered [1]\'s question: [2]' },
		13: { name: 'Voted on Question', template: '[0] found [1]\'s question [2]' }, // [2] = quote (e.g. useful)
		14: { name: 'Voted on Answer', template: '[0] found [1]\'s answer [2]' }, // [2] = quote (e.g. useful)
		15: { name: 'Commented on Answer', template: '[0] commented on [1]\'s answer: [2]' },
		16: { name: 'Started Discussion', template: '[0] started a discussion about [1]: [2]' },
		17: { name: 'Contributed to Discussion', template: '[0] contributed to discussion about [1]: [2]' },
		18: { name: 'Voted on Discussion Post', template: '[0] found [1]\'s post [2]' }, // [2] = quote (e.g. useful)
		19: { name: 'Liked Brand', template: '[0] likes [1]' } // [1] = brandName
	};

	function getTransactionTypeName(id: number): string {
		return transactionTypeInfo[id]?.name || `Unknown Action (${id})`;
	}

	// Function to generate descriptive text segments based on type and item data
	function getFeedItemDetailsSegments(item: FeedItem): TextSegment[] {
		const info = transactionTypeInfo[item.socialShoppingTransactionTypeId];
		const segments: TextSegment[] = [];

		const add = (content: string, style: TextSegment['style'], url?: string) => {
			if (content?.trim()) { // Avoid adding empty segments
				segments.push({ content: content.trim(), style, url });
			}
		};

		const city = item.cityName ?? 'somewhere';
		const productName = item.fullProductName?.trim() ?? 'a product';
		const brandName = item.brandName ?? 'a brand';
		const targetUser = item.targetUserName ?? 'someone';
		const quote = item.quote ?? '';
		const ratingValue = item.rating // Store rating value
		const ratingText = ratingValue ? `${ratingValue}/5` : '';
		const price = item.displayPrice ? `${item.displayPrice.amountInclusive} ${item.displayPrice.currency}` : '';
		const oauthProvider = item.oAuthProviderName ?? 'an external service';
		const search = item.searchString ?? 'something';
		const user = item.userName;

		try {
			switch (item.socialShoppingTransactionTypeId) {
				case 1: // Rated Product
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					add(' rated ', 'normal'); add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(' with ', 'normal'); add(ratingText, 'rating'); add(' stars', 'normal');
					break;
				case 2: // Created Account
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city'); add(' just registered', 'normal');
					break;
				case 3: // Ordered Product
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					add(' ordered ', 'normal'); add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(' for ', 'normal'); add(price, 'price');
					break;
				case 4: // Picked Up Product
					add(user, 'user'); add(' picked up ', 'normal'); add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(' for ', 'normal'); add(price, 'price'); add(' in store ', 'normal'); add(city, 'city');
					break;
				case 5: // Shipped Product
					add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(' for ', 'normal'); add(price, 'price'); add(' was sent to ', 'normal');
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					break;
				case 6: // Watched Product
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					add(' is looking at ', 'normal'); add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(' for ', 'normal'); add(price, 'price');
					break;
				case 7: // Searched
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					add(' is looking for ', 'normal'); add(`"${search}"`, 'quote');
					break;
				case 8: // Connected Account
					add(user, 'user'); add(' from ', 'normal'); add(city, 'city');
					add(' connected account via ', 'normal'); add(oauthProvider, 'brand');
					break;
				case 9: // Voted on Rating
				case 13: // Voted on Question
				case 14: // Voted on Answer
				case 18: // Voted on Discussion Post
					add(user, 'user'); add(' found ', 'normal'); add(targetUser, 'user');
					add(`'s ${info?.name.split(' ').pop()?.toLowerCase()} `, 'normal'); add(`"${quote}"`, 'quote', item.url);
					break;
				case 10: // Commented on Rating
				case 15: // Commented on Answer
					add(user, 'user'); add(' commented on ', 'normal'); add(targetUser, 'user');
					add(`'s ${info?.name.split(' ').pop()?.toLowerCase()}: `, 'normal'); add(`"${quote}"`, 'quote', item.url);
					break;
				case 11: // Asked Question
				case 16: // Started Discussion
				case 17: // Contributed to Discussion
					add(user, 'user'); add(` ${info?.name.toLowerCase()} about `, 'normal');
					add(brandName, 'brand'); add(productName, 'normal', item.url);
					add(': ', 'normal'); add(`"${quote}"`, 'quote');
					break;
				case 12: // Answered Question
					add(user, 'user'); add(' answered ', 'normal'); add(targetUser, 'user');
					add(`'s question: `, 'normal'); add(`"${quote}"`, 'quote', item.url);
					break;
				case 19: // Liked Brand
					add(user, 'user'); add(' likes ', 'normal'); add(brandName, 'brand', item.url);
					break;
				default:
					add(info?.name || `Unknown Action (${item.socialShoppingTransactionTypeId})`, 'normal');
			}
		} catch (err) {
			console.error("Error generating feed segments for item:", item, err);
			segments.push({ content: info?.name || `Unknown Action (${item.socialShoppingTransactionTypeId})`, style: 'normal' }); // Fallback
		}

		return segments;
	}

	async function loadFeed(page: number) {
		isLoading.set(true);
		error.set(null);
		try {
			const data = await fetchFeedItems(page, itemsPerPage);
			feedData.set(data);
			currentPage.set(page);
		} catch (e) {
			error.set('Failed to load feed items. Please try again later.');
			console.error(e);
		} finally {
			isLoading.set(false);
		}
	}

	onMount(() => {
		loadFeed($currentPage);
	});

	function formatDateTime(isoString: string): string {
		try {
			return new Date(isoString).toLocaleString();
		} catch {
			return 'Invalid Date';
		}
	}

	function goToPage(page: number) {
		if (page >= 1 && page <= ($feedData?.totalPages ?? 1)) {
			loadFeed(page);
		}
	}
</script>

<div class="container mx-auto p-4">
	<h1 class="text-2xl font-bold mb-4">Digitec Live Feed</h1>

	{#if $isLoading}
		<div class="flex justify-center items-center p-10">
			<span class="loading loading-spinner loading-lg"></span>
		</div>
	{:else if $error}
		<div role="alert" class="alert alert-error">
			<svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2 2m2-2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
			<span>Error! {$error}</span>
		</div>
	{:else if $feedData && $feedData.items.length > 0}
		<div class="overflow-x-auto">
			<table class="table table-zebra w-full">
				<tbody>
					{#each $feedData.items as item (item.id)}
						{@const detailsSegments = getFeedItemDetailsSegments(item)}
						<tr>
							<td class="w-1/6 align-top pt-3 pr-4">
								{#if item.imageUrl}
									<img 
										src={item.imageUrl.startsWith('//') ? `https:${item.imageUrl}` : item.imageUrl} 
										alt="Product image" 
										class="h-16 w-16 object-contain rounded mr-2 float-left"
										loading="lazy"
									/>
								{:else}
									<div class="h-16 w-16 float-left mr-2 bg-base-200 rounded"></div>
								{/if}
							</td>
							<td class="align-top pt-3">
								<div class="text-sm">
									{#each detailsSegments as segment (segment.content + segment.style)}
										<span class:font-bold={segment.style === 'user' || segment.style === 'price' || segment.style === 'rating' || segment.style === 'brand'}
											  class:text-accent={segment.style === 'price'}
											  class:text-info={segment.style === 'city'}
											  class:italic={segment.style === 'quote'}
											  class:link={segment.url !== undefined}
											  class:link-hover={segment.url !== undefined}
										>
											{#if segment.url}
												<a href={segment.url} target="_blank" rel="noopener noreferrer">{segment.content}</a>
											{:else}
												{segment.content}
											{/if}
										</span>
										{' '}
									{/each}
								</div>
								<div class="text-xs text-gray-500 mt-1">
									{formatDateTime(item.dateTime)}
									{#if item.cityName} in <span class="text-info font-medium">{item.cityName}</span>{/if}
								</div>
								{#if item.rating && [1].includes(item.socialShoppingTransactionTypeId)}
									<div class="rating rating-xs mt-1">
										{#each { length: 5 } as _, i}
											<input type="radio" name="rating-{item.id}" class="mask mask-star-2 bg-orange-400" disabled checked={item.rating === (i + 1)} />
										{/each}
									</div>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Pagination -->
		<div class="join mt-4 flex justify-center">
			<button class="join-item btn btn-sm" disabled={$currentPage <= 1} on:click={() => goToPage($currentPage - 1)}>«</button>
			<button class="join-item btn btn-sm btn-active">Page {$currentPage} / {$feedData.totalPages}</button>
			<button class="join-item btn btn-sm" disabled={$currentPage >= $feedData.totalPages} on:click={() => goToPage($currentPage + 1)}>»</button>
		</div>

	{:else}
		<p class="text-center p-10">No feed items found.</p>
	{/if}
</div>
