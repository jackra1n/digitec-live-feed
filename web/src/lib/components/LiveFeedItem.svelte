<script>
// @ts-nocheck
    export let item = {};
</script>

<div class="flex justify-between w-11/12 p-2 mt-2 rounded-md bg-surface-800 hover:bg-surface-900">

    <div class="flex">
    {#if item.socialShoppingTransactionTypeId == 1}

    <img src={item.imageUrl} alt={item.fullProductName} class="w-16 h-16 p-1 rounded-md mr-4 object-contain bg-white"/>
    <div>
        <p><strong>{item.userName}</strong> rated </p>
        <p>{item.brandName} {item.fullProductName}</p>
    </div>

    {:else if item.socialShoppingTransactionTypeId == 2}

    <p>{item.userName} from {item.cityName} registered as a new user</p>

    {:else if [3, 4].includes(item.socialShoppingTransactionTypeId)}

    <img src={item.imageUrl} alt={item.fullProductName} class="w-16 h-16 p-1 rounded-md mr-4 object-contain bg-white"/>
    <div>
        <p>
            <strong>{item.userName}</strong> 
            {#if item.cityName != null}
                from <strong>{item.cityName}</strong> 
            {/if} 

            {#if item.socialShoppingTransactionTypeId == 3}
                ordered
            {:else}
                picked up
            {/if}
        </p>
        {#if item.brandName != null}
        <p>{item.brandName} {item.fullProductName}</p>
        {:else}
        <p>{item.fullProductName}</p>
        {/if}
        
        {#if item.salesPrice != null}
        <p>CHF {item.salesPrice.amountIncl}</p>
        {/if}
    </div>

    {:else if item.socialShoppingTransactionTypeId == 5}

    <img src={item.imageUrl} alt={item.fullProductName} class="w-16 h-16 p-1 rounded-md mr-4 object-contain bg-white">
    <div>
        <p>{item.brandName} {item.fullProductName}</p>
        <p><strong>was shipped to</strong> {item.userName}</p>
    </div>

    {:else if item.socialShoppingTransactionTypeId == 19}

    <p>{item.userName} rated {item.brandName} {item.fullProductName} with {item.rating} stars</p>

    {:else}

    <p>Unknown transaction type: {item.socialShoppingTransactionTypeId}</p>

        
    {/if}
    </div>

    <p class="px-2 bg-secondary-600 rounded-full">{new Date(item.dateTime).toLocaleTimeString()}</p>
</div>