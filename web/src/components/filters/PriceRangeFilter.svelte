<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let minPrice = 0;
  export let maxPrice = 0;
  
  const dispatch = createEventDispatcher<{
    change: { min: number; max: number };
  }>();

  function handleChange() {
    dispatch('change', { min: minPrice, max: maxPrice });
  }
</script>

<div class="space-y-2">
  <label class="block text-sm font-medium text-gray-700">Price Range (CHF)</label>
  <div class="grid grid-cols-2 gap-4">
    <div>
      <label class="block text-xs text-gray-500">Min</label>
      <input
        type="number"
        bind:value={minPrice}
        min="0"
        on:change={handleChange}
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>
    <div>
      <label class="block text-xs text-gray-500">Max (0 for no limit)</label>
      <input
        type="number"
        bind:value={maxPrice}
        min="0"
        on:change={handleChange}
        class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
      />
    </div>
  </div>
</div>