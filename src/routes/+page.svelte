<script lang="ts">
  import Logo from "../components/ui/Logo.svelte";
  import Button from "../components/ui/Button.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  onMount(() => {
    goto("/onboarding");
  });
</script>

<div class="absolute inset-0 h-10 z-50" data-tauri-drag-region>
  <!-- Title bar area for dragging the window -->
</div>
<div
  class="flex flex-col items-center justify-center h-screen bg-background relative"
  data-tauri-drag-region
>
  <Logo class="size-24" />
</div>