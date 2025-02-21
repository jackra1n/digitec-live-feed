<script lang="ts">
  import { onMount } from 'svelte';
  import * as d3 from 'd3';
  import type { SalesData, FilterState } from '../types';
  import { salesData, cities, products } from '../data/mockData';

  export let filters: FilterState;

  let svg: SVGElement;
  let filteredData: SalesData[] = [];

  $: {
    filteredData = salesData.filter(sale => {
      const dateInRange = new Date(sale.date) >= new Date(filters.startDate) &&
                         new Date(sale.date) <= new Date(filters.endDate);
      const cityMatch = filters.cities.length === 0 || filters.cities.includes(sale.cityId);
      const productMatch = filters.products.length === 0 || filters.products.includes(sale.productId);
      
      return dateInRange && cityMatch && productMatch;
    });
    
    if (svg) updateChart();
  }

  function updateChart() {
    const width = 800;
    const height = 400;
    const margin = { top: 20, right: 20, bottom: 30, left: 40 };

    // Clear previous chart
    d3.select(svg).selectAll("*").remove();

    const x = d3.scaleBand()
      .range([margin.left, width - margin.right])
      .padding(0.1);

    const y = d3.scaleLinear()
      .range([height - margin.bottom, margin.top]);

    const chart = d3.select(svg)
      .attr("viewBox", `0 0 ${width} ${height}`);

    // Group data by city
    const groupedData = d3.group(filteredData, d => d.cityId);
    const cityTotals = Array.from(groupedData, ([cityId, sales]) => ({
      cityId,
      cityName: cities.find(c => c.id === cityId)?.name || '',
      total: d3.sum(sales, d => d.amount)
    }));

    x.domain(cityTotals.map(d => d.cityName));
    y.domain([0, d3.max(cityTotals, d => d.total) || 0]);

    // Add bars
    chart.selectAll("rect")
      .data(cityTotals)
      .join("rect")
      .attr("x", d => x(d.cityName) || 0)
      .attr("y", d => y(d.total))
      .attr("height", d => y(0) - y(d.total))
      .attr("width", x.bandwidth())
      .attr("fill", "steelblue");

    // Add axes
    chart.append("g")
      .attr("transform", `translate(0,${height - margin.bottom})`)
      .call(d3.axisBottom(x));

    chart.append("g")
      .attr("transform", `translate(${margin.left},0)`)
      .call(d3.axisLeft(y));
  }

  onMount(() => {
    updateChart();
  });
</script>

<div class="bg-white p-4 rounded-lg shadow-md">
  <h2 class="text-xl font-bold mb-4">Sales by City</h2>
  <svg bind:this={svg} class="w-full h-[400px]"></svg>
</div>