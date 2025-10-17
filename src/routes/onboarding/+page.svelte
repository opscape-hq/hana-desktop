<script lang="ts">
  import Logo from "../../components/ui/Logo.svelte";
  import Button from "../../components/ui/Button.svelte";
  import { goto } from "$app/navigation";

</script>

<div class="absolute inset-0 h-10 z-50" data-tauri-drag-region>
  <!-- Title bar area for dragging the window -->
</div>
<div
  class="flex flex-col items-center justify-center h-screen bg-background relative"
  data-tauri-drag-region
>
  <h1 class="text-3xl font-light">Welcome to Opscape</h1>
  <p class="mt-4">The future of SSH awaits</p>
  <div class="mt-4 flex gap-4 items-center">
    <!-- <Button variant="primary">Login in browser</Button>
    <Button variant="secondary">Skip for now</Button> -->
    <Button variant="primary" onClick={() => goto("/app")}>Proceed to setup</Button>
  </div>
  <!-- <p class="mt-4 text-sm text-muted-foreground max-w-sm text-center">Logging in allows you to use Opscape Cloud features including self-hosted Sync Servers.</p> -->
</div>
<div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 flex gap-2">
  <div class="w-2 h-2 bg-primary rounded-full"></div>
  <div class="w-2 h-2 bg-gray-400 rounded-full"></div>
  <div class="w-2 h-2 bg-gray-400 rounded-full"></div>
</div>