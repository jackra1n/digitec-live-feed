<script lang="ts">
  import type { DisplayPrice } from '../../types';
  
  export let price: DisplayPrice;
  
  const formatter = new Intl.NumberFormat('de-CH', {
    style: 'currency',
    currency: price.currency,
  });
</script>

<div>
  <div class="font-semibold">{formatter.format(price.amountInclusive)}</div>
  <div class="text-sm text-gray-500">{formatter.format(price.amountExclusive)} excl. VAT</div>
</div>