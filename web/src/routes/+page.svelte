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
        console.log(socialShopping);
        
        socialShopping = socialShopping.filter(filterDuplicateItems);
        liveFeedEntries = socialShopping.concat(liveFeedEntries);
    }

    // @ts-ignore
    const filterDuplicateItems = (item) => {
        return liveFeedEntries.filter((liveFeedItem) => {
            return liveFeedItem.userName === item.userName &&
            liveFeedItem.fullProductName === item.fullProductName &&
            liveFeedItem.dateTime === item.dateTime &&
            liveFeedItem.url === item.url;
        }).length === 0;
    }

    onMount(() => {
        fetchSocialShopping();
        setInterval(fetchSocialShopping, 30000);
    });
    
</script>

<div class="flex flex-col justify-center items-center h-[100vh] pt-4">
    <div class="relative overflow-hidden flex flex-col items-center max-h-[660px] rounded-[10px] border-[1px] border-gray-200 w-[576px] mx-auto p-4 bg-white bg-clip-border shadow-md shadow-blue-600 ">
        <div class="flex items-center justify-between rounded-t-3xl p-3 w-full">
            <div class="text-lg font-bold text-navy-700">
                Live Feed
            </div>
            <button class="linear rounded-[20px] bg-lightPrimary px-4 py-2 text-base font-medium text-brand-500 transition duration-200 hover:bg-gray-100 active:bg-gray-200">
                See all
            </button>
        </div>
        {#each liveFeedEntries as item}
            <LiveFeedItem {item} />
        {/each}
    </div>  
</div>


