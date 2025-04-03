<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchFeedItems } from '$lib/api';
	import type { FeedItem, PaginatedResponse } from '$lib/types';
	import { writable } from 'svelte/store';
	import FeedItemRow from '$lib/components/FeedItemRow.svelte';

	const feedData = writable<PaginatedResponse<FeedItem> | null>(null);
	const currentPage = writable(1);
	const itemsPerPage = 15;
	let isLoading = writable(true);
	let error = writable<string | null>(null);

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
						<FeedItemRow {item} />
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
