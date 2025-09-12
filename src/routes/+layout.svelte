<script lang="ts">
  let { children } = $props();
  import "../app.css";

  import { slide } from "svelte/transition";
</script>

<div class="flex flex-col h-screen">
  <!-- Titlebar -->
  <div class="select-none h-10 box-border text-sm flex bg-titlebar-background">
    <!-- Traffic lights space -->
    <div class="w-[74px]"></div>

    <div class="flex flex-1" data-tauri-drag-region>
      <div class="px-2 flex items-center rounded-t-lg bg-background">
        <span>root@192.168.0.1 — neofetch</span>
      </div>
    </div>
  </div>

  <div class="flex-1 bg-background" transition:slide>
    {@render children()}
  </div>
</div>
