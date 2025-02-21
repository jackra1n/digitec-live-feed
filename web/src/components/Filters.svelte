<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FilterState } from '../types';
  import { cities, products } from '../data/mockData';
  import MultiSelect from './MultiSelect.svelte';
  import BrandFilter from './filters/BrandFilter.svelte';
  import PriceRangeFilter from './filters/PriceRangeFilter.svelte';

  const dispatch = createEventDispatcher<{
    filterChange: FilterState;
  }>();

  let selectedCities: string[] = [];
  let selectedProducts: string[] = [];
  let selectedBrands: string[] = [];
  let startDate = '2024-01-01';
  let endDate = '2024-12-31';
  let priceRange = { min: 0, max: 0 };

  function updateFilters() {
    dispatch('filterChange', {
      cities: selectedCities,
      products: selectedProducts,
      brands: selectedBrands,
      startDate,
      endDate,
      priceRange
    });
  }

  function handleCityChange(event: CustomEvent<string[]>) {
    selectedCities = event.detail;
    updateFilters();
  }

  function handleProductChange(event: CustomEvent<string[]>) {
    selectedProducts = event.detail;
    updateFilters();
  }

  function handleBrandChange(event: CustomEvent<string[]>) {
    selectedBrands = event.detail;
    updateFilters();
  }

  function handlePriceChange(event: CustomEvent<{ min: number; max: number }>) {
    priceRange = event.detail;
    updateFilters();
  }
</script>

<div class="bg-white p-4 rounded-lg shadow-md">
  <h2 class="text-xl font-bold mb-4">Filters</h2>
  
  <div class="space-y-4">
    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Cities</label>
      <MultiSelect
        options={cities}
        bind:selected={selectedCities}
        placeholder="Select cities..."
        on:change={handleCityChange}
      />
    </div>

    <div>
      <label class="block text-sm font-medium text-gray-700 mb-1">Products</label>
      <MultiSelect
        options={products}
        bind:selected={selectedProducts}
        placeholder="Select products..."
        on:change={handleProductChange}
      />
    </div>

    <BrandFilter
      brands={['HP', 'ASUS', 'Dell', 'Apple']}
      bind:selectedBrands
      on:change={handleBrandChange}
    />

    <PriceRangeFilter
      bind:minPrice={priceRange.min}
      bind:maxPrice={priceRange.max}
      on:change={handlePriceChange}
    />

    <div class="grid grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700">Start Date</label>
        <input
          type="date"
          bind:value={startDate}
          on:change={updateFilters}
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700">End Date</label>
        <input
          type="date"
          bind:value={endDate}
          on:change={updateFilters}
          class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        />
      </div>
    </div>
  </div>
</div>