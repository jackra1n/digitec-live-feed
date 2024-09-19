<script>
// @ts-nocheck

    export let item;

    const action = {
        3: 'ordered by',
        4: 'picked up by',
        5: 'was sent to'
    }

</script>

<div class="flex items-center gap-3">
    <div class="flex h-16 w-16 items-center justify-center">
        <img class="h-full w-full rounded-xl object-contain"
            src="{item.imageUrl}"
            alt="{item.brandName} {item.fullProductName}"
        />
    </div>
    <div class="flex flex-col">
        <h5 class="text-base">
            { item.brandName } { item.fullProductName } 
        </h5>
        <p class="mt-1 text-sm font-normal text-gray-600">
            <span class="font-bold">{ item.userName }</span>
            at 
            <span class="font-bold">{ item.cityName }</span>
        </p>
    </div>
</div> 
<div class="mt-1 flex items-center justify-center text-navy-700">
    <div class="ml-1 flex items-center text-sm font-bold text-navy-700">
        <p/>
        <span>
            { Number(item.displayPrice.amountInclusive).toFixed(2) }<p class="ml-1">{ item.displayPrice.currency }</p>
        </span>
        <slot name="price" />
    </div>
    <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
        <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
    </div>
</div>