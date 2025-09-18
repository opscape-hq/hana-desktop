<script lang="ts">
  let { children } = $props();
  import "../app.css";

  import { slide } from "svelte/transition";

  import { invoke } from "@tauri-apps/api/core";
  let isMac: boolean = $state(false);

  // Fetch system info after component mounts
  import { onMount } from "svelte";
  import { Maximize, Minus, X } from "@lucide/svelte";
  onMount(async () => {
    isMac = await invoke("platform") === "macos";
  });

  import { getCurrentWindow } from "@tauri-apps/api/window";

  const appWindow = getCurrentWindow();
</script>

<div class="antialiased flex flex-col h-screen bg-titlebar-background">
  <!-- Titlebar -->
  <div class="select-none h-10 box-border text-sm flex">
    <!-- Traffic lights space -->
    <div class={isMac ? "w-[78px]" : "w-6"} data-tauri-drag-region></div>

    <div class="flex flex-1 pt-1" data-tauri-drag-region>
      <div class="px-2 flex items-center rounded-t-lg bg-background">
        <span>root@192.168.0.1 — neofetch</span>
      </div>
    </div>

    {#if !isMac}
      <div class="ml-auto flex">
        <button
          onclick={() => appWindow.minimize()}
          class="h-full w-10 hover:bg-white/20 flex justify-center items-center"
        >
          <Minus size={16} />
        </button>
        <button
          onclick={() => appWindow.toggleMaximize()}
          class="h-full w-10 hover:bg-white/20 flex justify-center items-center"
        >
          <Maximize size={16} />
        </button>
        <button
          onclick={() => appWindow.close()}
          class="h-full w-10 hover:bg-white/20 flex justify-center items-center"
        >
          <X size={16} />
        </button>
      </div>
    {/if}
  </div>

  <div class="flex-1 bg-background mx-1 mb-1 rounded-xl" transition:slide>
    {@render children()}
  </div>
</div>
