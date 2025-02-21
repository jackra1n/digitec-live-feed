<script lang="ts">
  import { format } from 'date-fns';
  import type { Transaction } from '../types';
  import TableImage from './table/TableImage.svelte';
  import TablePrice from './table/TablePrice.svelte';
  import ProductLink from './table/ProductLink.svelte';

  export let transactions: Transaction[] = [];
  
  let sortField: keyof Transaction = 'dateTime';
  let sortDirection: 'asc' | 'desc' = 'desc';

  $: sortedData = [...transactions].sort((a, b) => {
    const aValue = a[sortField];
    const bValue = b[sortField];
    
    const modifier = sortDirection === 'asc' ? 1 : -1;
    
    if (aValue < bValue) return -1 * modifier;
    if (aValue > bValue) return 1 * modifier;
    return 0;
  });

  function toggleSort(field: keyof Transaction) {
    if (sortField === field) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortField = field;
      sortDirection = 'desc';
    }
  }

  function getSortIcon(field: keyof Transaction) {
    if (sortField !== field) return '↕️';
    return sortDirection === 'asc' ? '↑' : '↓';
  }
</script>

<div class="bg-white rounded-lg shadow-md overflow-hidden">
  {#if transactions.length === 0}
    <div class="flex flex-col items-center justify-center p-8 text-gray-500">
      <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
      </svg>
      <p class="text-lg font-medium">No matching transactions found</p>
      <p class="text-sm">Try adjusting your filters to see more results</p>
    </div>
  {:else}
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th></th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Product</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer" on:click={() => toggleSort('brandName')}>
              Brand {getSortIcon('brandName')}
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer" on:click={() => toggleSort('cityName')}>
              City {getSortIcon('cityName')}
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Price</th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer" on:click={() => toggleSort('dateTime')}>
              Date {getSortIcon('dateTime')}
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          {#each sortedData as item}
            <tr class="hover:bg-gray-50">
              <td>
                <div class="ml-4">
                  <TableImage src={item.imageUrl} alt={item.fullProductName} />
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="flex items-center">
                  <div class="text-sm font-bold">
                    <a href="https://digitec.ch{item.url}" target="_blank" class="text-gray-900 hover:text-blue-600">{item.fullProductName}</a>
                  </div>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{item.brandName}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{item.cityName}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <TablePrice price={item.displayPrice} />
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">
                  {format(new Date(item.dateTime), 'dd.MM.yyyy')}
                </div>
                <div class="text-sm text-gray-500">
                  {format(new Date(item.dateTime), 'HH:mm')}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>