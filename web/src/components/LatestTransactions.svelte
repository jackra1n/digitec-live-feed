<script lang="ts">
  import { format } from 'date-fns';
  import type { Transaction } from '../types';
  
  export let transactions: Transaction[] = [];

  function formatPrice(price: number, currency: string): string {
    return new Intl.NumberFormat('de-CH', {
      style: 'currency',
      currency: currency
    }).format(price);
  }
</script>

<div class="bg-white rounded-lg shadow-md p-6">
  {#if transactions.length === 0}
    <div class="flex flex-col items-center justify-center py-8 text-gray-500">
      <svg class="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <p class="text-lg font-medium">No transactions yet</p>
      <p class="text-sm">New transactions will appear here as they happen</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each transactions as transaction}
        <div class="flex items-center space-x-4">
          <img 
            src={transaction.imageUrl} 
            alt={transaction.fullProductName}
            class="w-12 h-12 object-contain"
          />
          <p class="text-gray-800">
            <span class="font-semibold text-blue-600">{transaction.userName}</span>
            bought a
            <a 
              href="https://digitec.ch{transaction.url}"
              target="_blank"
              class="group inline-flex items-center"
            >
              <span class="font-bold text-gray-900 group-hover:text-blue-600">
                {transaction.brandName}
              </span>
              <span class="mx-1">·</span>
              <span class="text-gray-700 group-hover:text-blue-600">
                {transaction.fullProductName}
              </span>
            </a>
            for
            <span class="font-semibold text-blue-600">
              {formatPrice(transaction.displayPrice.amountInclusive, transaction.displayPrice.currency)}
            </span>
            in
            <span class="font-medium text-gray-700">{transaction.cityName}</span>
            <span 
              class="text-sm text-gray-500 ml-2 cursor-help"
              title={format(new Date(transaction.dateTime), 'dd.MM.yyyy HH:mm:ss')}
            >
              {format(new Date(transaction.dateTime), 'HH:mm')}
            </span>
          </p>
        </div>
      {/each}
    </div>
  {/if}
</div>