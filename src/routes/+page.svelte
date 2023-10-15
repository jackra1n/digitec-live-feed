<script>
    import { onMount } from 'svelte';

    /**
	 * @type {any[]}
	 */
    let live_feed_entries = [];

    onMount(async () => {
        let response = await fetch('/api/liveshopping');
        let socialShopping = await response.json();
        live_feed_entries = [live_feed_entries, ...socialShopping.items];
    });


    
</script>

<div class="h-screen flex justify-center items-center">
    <div class="border-2 border-sky-200 rounded-lg p-2">
        <div class="flex justify-center p-2 my-2">
            <h1 class="text-2xl font-semibold">Live feed</h1>
        </div>
        <div>
            {#each live_feed_entries as entry}
            <div class="flex border-2 border-sky-200 p-2 mt-2 rounded-md bg-slate-200">
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

