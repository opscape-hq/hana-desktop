<script lang="ts">
  let { children } = $props();
  import "../app.css";

  import { slide } from "svelte/transition";
</script>

<div class="flex flex-col h-screen">
  <!-- Titlebar -->
  <div class="select-none h-10 border-b box-border text-sm flex">
    <!-- Traffic lights space -->
    <div class="w-[74px] border-r"></div>

    <div class="flex flex-1" data-tauri-drag-region>
      <div class="px-2 flex items-center border-r">
        <span>root@192.168.0.1 — neofetch</span>
      </div>
    </div>
  </div>

  <div class="flex-1 bg-red-500" transition:slide>
    {@render children()}
  </div>
</div>
