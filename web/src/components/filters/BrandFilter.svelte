<script lang="ts">
  import MultiSelect from '../MultiSelect.svelte';
  import { createEventDispatcher } from 'svelte';
  
  export let brands: string[] = [];
  export let selectedBrands: string[] = [];
  
  const dispatch = createEventDispatcher<{
    change: string[];
  }>();

  const brandOptions = [...new Set(brands)].map(brand => ({
    id: brand,
    name: brand
  }));

  function handleChange(event: CustomEvent<string[]>) {
    selectedBrands = event.detail;
    dispatch('change', selectedBrands);
  }
</script>

<div>
  <label class="block text-sm font-medium text-gray-700 mb-1">Brands</label>
  <MultiSelect
    options={brandOptions}
    bind:selected={selectedBrands}
    placeholder="Select brands..."
    on:change={handleChange}
  />
</div>