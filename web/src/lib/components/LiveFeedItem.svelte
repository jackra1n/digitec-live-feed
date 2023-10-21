<script>
    // @ts-nocheck
    export let item = {};
</script>

<div class="flex h-full w-full items-start justify-between rounded-md border-[1px] border-[transparent] bg-white mt-2 px-3 py-[20px] transition-all duration-150 hover:border-blue-700 hover:shadow-blue-600 hover:shadow-md">    
    <div class="flex items-center gap-3">
        <div class="flex h-16 w-16 items-center justify-center">
        <img
            class="h-full w-full rounded-xl object-contain"
            src="{item.imageUrl}"
            alt="{item.brandName} {item.fullProductName}"
        />
        </div>
        <div class="flex flex-col">
        <h5 class="text-base font-bold text-navy-700">
            { item.brandName } { item.fullProductName }
        </h5>
        <p class="mt-1 text-sm font-normal text-gray-600">
            { item.userName }
        </p>
        </div>
    </div> 
    <div class="mt-1 flex items-center justify-center text-navy-700">
        <div class="ml-1 flex items-center text-sm font-bold text-navy-700">
        <p>   </p>
        { item.displayPrice.amountInclusive }<p class="ml-1">{ item.displayPrice.currency }</p>
        </div>
        <div class="ml-2 flex items-center text-sm font-normal text-gray-600">
        <p>{new Date(item.dateTime).toLocaleTimeString()}</p>
        </div>
    </div>
</div>