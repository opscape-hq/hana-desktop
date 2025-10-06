<script lang="ts">
  import { Plus } from "@lucide/svelte";
  import TabButton from "./TabButton.svelte";
  import type { Tab } from "../types/tab";

  interface Props {
    tabs: Tab[];
    activeTabId: string;
    onTabSwitch: (tabId: string) => void;
    onTabClose: (tabId: string) => void;
    onAddTab: () => void;
  }

  let { tabs, activeTabId, onTabSwitch, onTabClose, onAddTab }: Props = $props();

  let tabBarElement: HTMLElement;
  let blueBarPosition = $state({ left: 0, width: 0 });

  /**
   * Calculates and updates the position of the blue indicator bar
   * based on the currently active tab's position and width
   */
  function updateBlueBarPosition() {
    if (!tabBarElement) return;

    const activeTabIndex = tabs.findIndex((tab) => tab.id === activeTabId);
    if (activeTabIndex === -1) return;

    const tabElements = tabBarElement.querySelectorAll("[data-tab-id]");
    const activeTabElement = tabElements[activeTabIndex] as HTMLElement;

    if (activeTabElement) {
      const tabBarRect = tabBarElement.getBoundingClientRect();
      const activeTabRect = activeTabElement.getBoundingClientRect();

      blueBarPosition = {
        left: activeTabRect.left - tabBarRect.left,
        width: activeTabRect.width,
      };
    }
  }

  /**
   * Reactive effect that updates the blue bar position
   * whenever the active tab or tabs array changes
   */
  $effect(() => {
    activeTabId;
    tabs;
    setTimeout(updateBlueBarPosition, 50);
  });
</script>

<div
  class="relative flex flex-1 pt-1 gap-2"
  data-tauri-drag-region
  bind:this={tabBarElement}
>
  {#each tabs as tab}
    <TabButton
      {tab}
      isActive={activeTabId === tab.id}
      onSwitch={() => onTabSwitch(tab.id)}
      onClose={() => onTabClose(tab.id)}
    />
  {/each}

  <!-- Animated blue bar indicator -->
  <div
    class="absolute bottom-0 h-0.5 bg-blue-500 rounded-full transition-all duration-500 ease-out z-10"
    style="left: {blueBarPosition.left}px; width: {blueBarPosition.width}px;"
  ></div>

  <!-- Add new tab button -->
  <button
    onclick={onAddTab}
    class="flex items-center justify-center h-full aspect-square rounded-lg bg-tab hover:bg-tab-accent transition-all duration-200 opacity-60 hover:opacity-100"
    title="New Terminal"
  >
    <Plus size={14} />
  </button>
</div>
