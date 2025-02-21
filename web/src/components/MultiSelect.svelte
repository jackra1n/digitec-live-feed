<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let options: { id: string; name: string }[] = [];
  export let selected: string[] = [];
  export let placeholder = 'Select options...';
  
  let isOpen = false;
  let searchTerm = '';
  
  const dispatch = createEventDispatcher<{
    change: string[];
  }>();

  $: filteredOptions = options.filter(option => 
    option.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  function toggleOption(id: string) {
    const index = selected.indexOf(id);
    if (index === -1) {
      selected = [...selected, id];
    } else {
      selected = selected.filter(i => i !== id);
    }
    dispatch('change', selected);
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.multi-select')) {
      isOpen = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="multi-select relative">
  <div
    class="min-h-[42px] p-2 border rounded-md bg-white cursor-pointer flex flex-wrap gap-2 items-center"
    on:click={() => isOpen = !isOpen}
  >
    {#if selected.length === 0}
      <span class="text-gray-500">{placeholder}</span>
    {:else}
      {#each selected as id}
        <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded-md text-sm">
          {options.find(o => o.id === id)?.name}
        </span>
      {/each}
    {/if}
  </div>

  {#if isOpen}
    <div class="absolute z-10 mt-1 w-full bg-white border rounded-md shadow-lg">
      <input
        type="text"
        placeholder="Search..."
        bind:value={searchTerm}
        class="w-full p-2 border-b"
        on:click|stopPropagation
      />
      <div class="max-h-60 overflow-y-auto">
        {#each filteredOptions as option}
          <div
            class="p-2 hover:bg-gray-100 cursor-pointer flex items-center gap-2"
            on:click|stopPropagation={() => toggleOption(option.id)}
          >
            <input
              type="checkbox"
              checked={selected.includes(option.id)}
              class="h-4 w-4"
            />
            <span>{option.name}</span>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>