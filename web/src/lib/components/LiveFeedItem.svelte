<script>
  import LiveFeedItemClean from "./LiveFeedItemClean.svelte";

    // @ts-nocheck
    export let item = {};
</script>

<div class="flex h-full w-full items-start justify-between rounded-md border-[1px] border-[transparent] bg-white mt-2 px-3 py-[20px] transition-all duration-150 hover:border-blue-700 hover:shadow-blue-600 hover:shadow-sm">
    {#if item.socialShoppingTransactionTypeId === 1}

    review

    {:else if item.socialShoppingTransactionTypeId === 2}

    <div class="flex items-center gap-3">
        <div class="flex h-16 w-16 items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-12 h-12">
                <path stroke-linecap="round" stroke-linejoin="round" d="M17.982 18.725A7.488 7.488 0 0012 15.75a7.488 7.488 0 00-5.982 2.975m11.963 0a9 9 0 10-11.963 0m11.963 0A8.966 8.966 0 0112 21a8.966 8.966 0 01-5.982-2.275M15 9.75a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
        </div>
        <div class="flex flex-col">
        <p class="mt-1 text-sm font-normal text-gray-600">
            <span class="font-bold">{ item.userName }</span>
            {#if item.cityName}
                from <span class="font-bold">{ item.cityName }</span>
            {/if}
            registered an account
        </p>
        </div>
    </div> 
    <div class="mt-1 flex items-center justify-center text-navy-700">
        <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
            <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
        </div>
    </div>

    {:else if [3,4,5].includes(item.socialShoppingTransactionTypeId)}

        <LiveFeedItemClean {item}/>

    {:else if item.socialShoppingTransactionTypeId === 9}

    <div class="flex items-center gap-3">
        <div class="flex h-16 w-16 items-center justify-center">
            {#if item.voteTypeId == 1}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6.633 10.5c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 012.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 00.322-1.672V3a.75.75 0 01.75-.75A2.25 2.25 0 0116.5 4.5c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 01-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 00-1.423-.23H5.904M14.25 9h2.25M5.904 18.75c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 01-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 10.203 4.167 9.75 5 9.75h1.053c.472 0 .745.556.5.96a8.958 8.958 0 00-1.302 4.665c0 1.194.232 2.333.654 3.375z" />
            </svg>
            {:else if item.voteTypeId == 2}
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 15h2.25m8.024-9.75c.011.05.028.1.052.148.591 1.2.924 2.55.924 3.977a8.96 8.96 0 01-.999 4.125m.023-8.25c-.076-.365.183-.75.575-.75h.908c.889 0 1.713.518 1.972 1.368.339 1.11.521 2.287.521 3.507 0 1.553-.295 3.036-.831 4.398C20.613 14.547 19.833 15 19 15h-1.053c-.472 0-.745-.556-.5-.96a8.95 8.95 0 00.303-.54m.023-8.25H16.48a4.5 4.5 0 01-1.423-.23l-3.114-1.04a4.5 4.5 0 00-1.423-.23H6.504c-.618 0-1.217.247-1.605.729A11.95 11.95 0 002.25 12c0 .434.023.863.068 1.285C2.427 14.306 3.346 15 4.372 15h3.126c.618 0 .991.724.725 1.282A7.471 7.471 0 007.5 19.5a2.25 2.25 0 002.25 2.25.75.75 0 00.75-.75v-.633c0-.573.11-1.14.322-1.672.304-.76.93-1.33 1.653-1.715a9.04 9.04 0 002.86-2.4c.498-.634 1.226-1.08 2.032-1.08h.384" />
            </svg>
            {/if}
        </div>
        <div class="flex flex-col">
        <p class="mt-1 text-sm font-normal text-gray-600">
            <span class="italic">"{item.quote}"</span>
            
            {#if item.voteTypeId == 1}
                upvoted by
            {:else if item.voteTypeId == 2}
                downvoted by
            {/if}
            <span class="font-bold">{ item.userName }</span>
            {#if item.cityName}
                from <span class="font-bold">{ item.cityName }</span>
            {/if}
        </p>
        </div>
    </div> 
    <div class="mt-1 flex items-center justify-center text-navy-700">
        <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
            <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
        </div>
    </div>

    {:else if item.socialShoppingTransactionTypeId === 10}

        reply to review

    {:else if item.socialShoppingTransactionTypeId === 11}

    <div class="flex items-center gap-3">
        <div class="flex h-16 w-16 items-center justify-center">
            <img
                class="h-full w-full rounded-xl object-contain"
                src="{item.imageUrl}"
                alt="{item.brandName} {item.fullProductName}"
            />
        </div>
        <div class="flex flex-col">
        <p class="mt-1 text-sm font-normal text-gray-600">
            <span class="italic">"{item.quote}"</span>
            
            asked by
            <span class="font-bold">{ item.userName }</span>
            {#if item.cityName}
                from <span class="font-bold">{ item.cityName }</span>
            {/if}
        </p>
        </div>
    </div> 
    <div class="mt-1 flex items-center justify-center text-navy-700">
        <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
            <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
        </div>
    </div>

    {:else if item.socialShoppingTransactionTypeId === 12}

    <div class="flex items-center gap-3">
        <div class="flex h-16 w-16 items-center justify-center">
            <img
                class="h-full w-full rounded-xl object-contain"
                src="{item.imageUrl}"
                alt="{item.brandName} {item.fullProductName}"
            />
        </div>
        <div class="flex flex-col">
        <p class="mt-1 text-sm font-normal">
            <span class="italic">"{item.quote}"</span>
            
            answered by
            <span class="font-bold">{ item.userName }</span>
            {#if item.cityName}
                from <span class="font-bold">{ item.cityName }</span>
            {/if}
            to <a href="https://digitec.ch{item.url}" class="font-bold">{ item.targetUserName }</a>
        </p>
        </div>
    </div> 
    <div class="mt-1 flex items-center justify-center text-navy-700">
        <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
            <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
        </div>
    </div>

    {:else }

        not implemented type
    
    {/if}
</div>