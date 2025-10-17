<script lang="ts">
  import { onMount } from "svelte";
  import gsap from "gsap";
  import { animation } from "../states/tabs.svelte";
  import type { Tab } from "../types/tab";
  import TerminalTab from "./TerminalTab.svelte";

  interface Props {
    tabs: Tab[];
    activeTabId: string;
    isTransitioning: boolean;
    onUpdateTabState: (tabId: string, newState: any) => void;
    children?: any;
  }

  let {
    tabs,
    activeTabId,
    isTransitioning,
    onUpdateTabState,
    children,
  }: Props = $props();

  let containerElement: HTMLElement;

  /**
   * Calculates the horizontal translation offset for the sliding animation
   * Each tab takes up 100% width, so we multiply by the active tab index
   */
  const activeTabIndex = $derived(
    tabs.findIndex((tab) => tab.id === activeTabId)
  );

  // Gap between tabs in pixels
  const gapSize = 16; // Adjust this value as needed
  
  /**
   * Calculate the translate value in pixels for GSAP
   * We need to get the actual container width and calculate pixel values
   * Formula: -(index * containerWidth + index * gap)
   */
  const translateXPx = $derived.by(() => {
    if (!containerElement) return 0;
    const containerWidth = containerElement.offsetWidth; // Width of one tab
    return -(activeTabIndex * (containerWidth + gapSize));
  });
  
  /**
   * Calculate transform origin to center on the active tab
   * This ensures scaling happens from the center of the active tab
   */
  const transformOrigin = $derived(`calc(${activeTabIndex * 100}% + ${activeTabIndex * gapSize}px + 50%)`);

  /**
   * Set up GSAP animations for both pressure-based scaling and tab switching
   * Using GSAP for everything avoids CSS transition conflicts
   */
  onMount(() => {
    if (containerElement) {
      // Watch for changes in animation.zoomOut and smoothly apply scale
      $effect(() => {
        const currentZoom = animation.zoomOut;
        const targetScale = 1 - currentZoom * 0.2;
        
        // Use GSAP for ultra-smooth scaling without CSS transition lag
        gsap.to(containerElement, {
          scale: targetScale,
          duration: 0.15, // Fast response to pressure changes
          ease: "power1.out",
          overwrite: "auto", // Automatically interrupt ongoing animations
        });
      });

      // Watch for tab changes and animate translateX with GSAP
      $effect(() => {
        // Trigger on translateX changes (which is derived from activeTabIndex)
        const currentTranslateX = translateXPx;
        
        // Animate the horizontal slide between tabs
        gsap.to(containerElement, {
          x: currentTranslateX,
          duration: 0.5,
          ease: "elastic.out(0.6, 0.9)",
          // Don't overwrite scale animations, only x
          overwrite: false,
        });
      });
    }
  });
</script>

<div class="relative h-full overflow-hidden" style="">
  <!-- Container that slides horizontally to show different tabs -->
  <!-- GSAP handles both scale AND translateX - no CSS transitions needed -->
  <div
    bind:this={containerElement}
    class="flex h-full transform-gpu gap-4"
    style="transform-origin: {transformOrigin} center;"
  >
    {#each tabs as tab}
      <div class="w-full h-full flex-shrink-0 relative rounded-xl overflow-hidden bg-background">
        {#if tab.id === "home"}
          <!-- Welcome tab content (from +page.svelte) -->
          {@render children()}
        {:else if tab.type === "dynamic"}
          <!-- Dynamic terminal tabs with internal navigation -->
          <TerminalTab {tab} {onUpdateTabState} />
        {/if}
      </div>
    {/each}
  </div>
</div>
