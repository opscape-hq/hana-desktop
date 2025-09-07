<script lang="ts">
  let { children } = $props();
  import "../app.css";

  import { slide } from 'svelte/transition';
</script>

<div transition:slide>
  {@render children()}
</div>