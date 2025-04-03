<script lang="ts">
    import type { FeedItem } from "$lib/types";
    import { getFeedItemDetailsSegments, formatDateTime } from "$lib/utils";

    export let item: FeedItem;

    $: detailsSegments = getFeedItemDetailsSegments(item);

</script>

<tr>
    <td class="w-1/10 align-top pt-3 pr-4">
        {#if item.imageUrl}
            <img 
                src={item.imageUrl.startsWith('//') ? `https:${item.imageUrl}` : item.imageUrl} 
                class="h-16 w-16 object-cover rounded mr-2 float-left"
                loading="lazy"
            />
        {:else}
            <div class="h-16 w-16 float-left mr-2 bg-base-200 rounded"></div> <!-- Placeholder -->
        {/if}
    </td>
    <td class="align-top pt-3">
        <div class="text-sm">
            {#each detailsSegments as segment (segment.content + segment.style)} 
                <span class:font-bold={segment.style === 'user' || segment.style === 'price' || segment.style === 'rating' || segment.style === 'brand'}
                      class:text-accent={segment.style === 'price'}
                      class:text-info={segment.style === 'city'}
                      class:italic={segment.style === 'quote'}
                      class:link={segment.url !== undefined && segment.style !== 'brand' && segment.style !== 'user'}
                      class:link-hover={segment.url !== undefined && segment.style !== 'brand' && segment.style !== 'user'}
                      class:font-semibold={segment.style === 'brand' || segment.style === 'user'}
                >
                    {#if segment.url}
                        <a href={segment.url} 
                           target="_blank" 
                           rel="noopener noreferrer" 
                           class:link-primary={segment.style === 'link' || segment.style === 'brand'} 
                           class:link-secondary={segment.style === 'user'}>{segment.content}</a>
                    {:else}
                        {segment.content}
                    {/if}
                </span>
                {' '}
            {/each}
        </div>
        <div class="text-xs text-gray-500 mt-1">
            {formatDateTime(item.dateTime)}
            {#if item.cityName} in <span class="text-info font-medium">{item.cityName}</span>{/if}
        </div>
        {#if item.rating && [1].includes(item.socialShoppingTransactionTypeId)} 
            <div class="rating rating-xs mt-1">
                {#each { length: 5 } as _, i}
                    <input type="radio" name="rating-{item.id}" class="mask mask-star-2 bg-orange-400" disabled checked={item.rating === (i + 1)} />
                {/each}
            </div>
        {/if}
    </td>
</tr> 