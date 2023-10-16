<script>
    import { onMount } from 'svelte';
    import LiveFeedItem from '$lib/components/LiveFeedItem.svelte';

    /**
	 * @type {any[]}
	 */
    let liveFeedEntries = [];
    let countdown = 30;

    const fetchSocialShopping = async () => {
        let response = await fetch('/api/liveshopping');
        let socialShopping = await response.json();
        console.log(socialShopping);
        liveFeedEntries = socialShopping.items.concat(liveFeedEntries);
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
    <div class="w-4/5 max-w-screen-md border-2 bg-sky-50 border-sky-200 rounded-lg p-2">
        <div class="flex flex-col items-center p-2 my-2">
            <h1 class="text-2xl font-semibold">Live feed</h1>
            <p>Refresh in: {countdown}</p>
        </div>
        <div class="overflow-y-auto flex flex-col items-center">
            {#each liveFeedEntries as item}
                <LiveFeedItem {item} />
            {/each}
        </div>

    </div>
</div>

