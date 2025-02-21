<script lang="ts">
  export let src: string;
  export let alt: string;
</script>

<div class="w-20 h-16 flex items-center justify-center">
  <img {src} {alt} class="max-w-full max-h-full object-contain" />
</div>