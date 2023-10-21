<script>
    import { onMount } from 'svelte';
    import { PUBLIC_BACKEND_URL } from '$env/static/public';
    import LiveFeedItem from '$lib/components/LiveFeedItem.svelte';

    /**
	 * @type {any[]}
	 */
    let liveFeedEntries = [];

    const fetchSocialShopping = async () => {
        let response = await fetch(PUBLIC_BACKEND_URL + '/api/v1/live-feed/');
        let socialShopping = await response.json();
        liveFeedEntries = socialShopping.items.concat(liveFeedEntries);
    }

    onMount(() => {
        fetchSocialShopping();
        setInterval(fetchSocialShopping, 30000);
    });
    
</script>

<div class="max-h-screen flex justify-center my-4">
    <div class="w-4/5 max-w-screen-md bg-sky-100 border border-sky-600 rounded-lg p-2">
        <div class="flex items-center p-2 my-2">
            <div class="w-full flex flex-col items-center">
                <h1 class="text-2xl font-semibold">Live feed</h1>
            </div>
            <button type="button" class="btn variant-filled-secondary">Pause Fetching</button>
        </div>
        <div class="overflow-y-auto flex flex-col items-center">
            {#each liveFeedEntries as item}
                <LiveFeedItem {item} />
            {/each}
        </div>

    </div>
</div>

