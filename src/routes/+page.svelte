<script>
    import { onMount } from 'svelte';

    /**
	 * @type {any[]}
	 */
    let live_feed_entries = [];
    let countdown = 30;

    const fetchSocialShopping = async () => {
        let response = await fetch('/api/liveshopping');
        let socialShopping = await response.json();
        live_feed_entries = [...socialShopping.items, ...live_feed_entries]
        console.log(live_feed_entries.length);
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
            {#each live_feed_entries as entry}
            <div class="flex border-2 border-sky-200 p-2 mt-2 rounded-md bg-sky-200">
                <img src={entry.imageUrl} alt={entry.fullProductName} class="w-16 h-16 p-1 rounded-md mr-4 object-contain bg-white"/>
                <div>
                    <p><strong>{entry.userName}</strong> from <strong>{entry.cityName}</strong> bought</p>
                    {#if entry.brandName != null}
                        <p>{entry.brandName} - {entry.fullProductName}</p>
                    {:else}
                        <p>{entry.fullProductName}</p>
                    {/if}
                    {#if entry.salesPrice != null}
                        <p>CHF {entry.salesPrice.amountIncl}</p>
                    {/if}
                </div>
            </div>
            {/each}
        </div>

    </div>
</div>

