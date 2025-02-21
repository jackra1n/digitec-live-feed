<script lang="ts">
  import Filters from './components/Filters.svelte';
  import FilteredDataTable from './components/FilteredDataTable.svelte';
  import LatestTransactions from './components/LatestTransactions.svelte';
  import type { FilterState, Transaction } from './types';

  let filters: FilterState = {
    cities: [],
    products: [],
    brands: [],
    startDate: '2024-01-01',
    endDate: '2024-12-31',
    priceRange: {
      min: 0,
      max: 0 // 0 means no upper limit
    }
  };

  // Example transaction data
  const transactions: Transaction[] = [{
    id: "385246523",
    userName: "P.",
    cityName: "Neerach",
    dateTime: "2024-12-28T17:02:57.8071000Z",
    imageUrl: "//static.digitecgalaxus.ch/productimages/1/7/2/4/0/9/9/9/7/0/5/0/5/7/9/8/7/4/9/84171927-89cd-4392-ba0a-6b78e0f779ee_cropped.jpg",
    brandName: "HP",
    fullProductName: "975 Dual Mode",
    displayPrice: {
      amountInclusive: 88.9,
      amountExclusive: 82.24,
      currency: "CHF"
    },
    url: "/en/s1/product/hp-975-dual-mode-ch-wireless-keyboard-17731911"
  },
  {
    id: "513322628",
    userName: "J.",
    cityName: "Bülach",
    dateTime: "2024-12-29T16:16:25.7146000Z",
    imageUrl: "//static.digitecgalaxus.ch/productimages/9/1/5/1/7/6/6/6/2/2/7/5/8/7/9/5/5/8/3/c56de9b4-ce51-4b68-9c1e-228f2bd6393a_cropped.jpg",
    brandName: "ASUS",
    fullProductName: "TUF Gaming VG27AQML1A",
    displayPrice: {
      amountInclusive: 349,
      amountExclusive: 322.85,
      currency: "CHF"
    },
    url: "/en/s1/product/asus-tuf-gaming-vg27aqml1a-2560-x-1440-pixels-27-monitor-36903455"
  }];

  function handleFilterChange(event: CustomEvent<FilterState>) {
    filters = event.detail;
  }

  $: filteredTransactions = transactions.filter(transaction => {
    const priceMatch = transaction.displayPrice.amountInclusive >= filters.priceRange.min &&
                      (filters.priceRange.max === 0 || transaction.displayPrice.amountInclusive <= filters.priceRange.max);
    const brandMatch = filters.brands.length === 0 || filters.brands.includes(transaction.brandName);
    const cityMatch = filters.cities.length === 0 || filters.cities.includes(transaction.cityName);
    const dateMatch = new Date(transaction.dateTime) >= new Date(filters.startDate) &&
                     new Date(transaction.dateTime) <= new Date(filters.endDate);

    return priceMatch && brandMatch && cityMatch && dateMatch;
  });
</script>

<main class="min-h-screen bg-gray-100 p-8">
  <div class="max-w-7xl mx-auto space-y-8">
    <h1 class="text-3xl font-bold">Live Feed Viewer</h1>
    
    <div>
      <h2 class="text-2xl font-bold mb-4">Latest Transactions</h2>
      <LatestTransactions transactions={transactions} />
    </div>

    <h2 class="text-2xl font-bold mb-4">Filtered Data</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
      <div class="md:col-span-1">
        <Filters on:filterChange={handleFilterChange} />
      </div>
      
      <div class="md:col-span-3">
        <FilteredDataTable transactions={filteredTransactions} />
      </div>
    </div>
  </div>
</main>