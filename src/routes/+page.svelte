<script>
    import { onMount } from 'svelte';
    import LiveFeedItem from '$lib/components/LiveFeedItem.svelte';

    /**
	 * @type {any[]}
	 */
    let live_feed_entries = [];
    let countdown = 30;

    const fetchSocialShopping = async () => {
        let response = await fetch('/api/liveshopping');
        let socialShopping = await response.json();
        console.log(socialShopping);
        live_feed_entries = live_feed_entries.concat(socialShopping.items.reverse());
    }

    const updateCountdown = () => {
        countdown--;
        if (countdown == 0) {
            countdown = 30;
        }
    }

    onMount(() => {
        fetchSocialShopping();
        setInterval(updateCountdown, 1000);
        setInterval(fetchSocialShopping, 30000);
    });
    
</script>

<div class="max-h-screen flex justify-center my-4">
    <div class="border-2 bg-sky-50 border-sky-200 rounded-lg p-2">
        <div class="flex flex-col items-center p-2 my-2">
            <h1 class="text-2xl font-semibold">Live feed</h1>
            <p>Refresh in: {countdown}</p>
        </div>
        <div class="max-h-min overflow-y-auto">
            {#each live_feed_entries as item}
                <LiveFeedItem {item} />
            {/each}
        </div>

    </div>
</div>

