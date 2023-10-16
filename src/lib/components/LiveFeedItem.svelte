<script>
    export let item = {};
</script>

<div class="flex border-2 border-sky-200 p-2 mt-2 rounded-md bg-sky-200">
    <img src={item.imageUrl} alt={item.fullProductName} class="w-16 h-16 p-1 rounded-md mr-4 object-contain bg-white"/>
    <div>
        <p><strong>{item.userName}</strong> from <strong>{item.cityName}</strong> bought</p>
        {#if item.brandName != null}
            <p>{item.brandName} - {item.fullProductName}</p>
        {:else}
            <p>{item.fullProductName}</p>
        {/if}
        {#if item.salesPrice != null}
            <p>CHF {item.salesPrice.amountIncl}</p>
        {/if}
    </div>
</div>