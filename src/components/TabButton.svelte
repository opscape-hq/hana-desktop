<script lang="ts">
  import { X } from "@lucide/svelte";
  import { onMount } from "svelte";
  import gsap from "gsap";
  import type { Tab } from "../types/tab";
  import { animation } from "../states/tabs.svelte";

  interface Props {
    tab: Tab;
    isActive: boolean;
    onSwitch: () => void;
    onClose: () => void;
  }

  let { tab, isActive, onSwitch, onClose }: Props = $props();

  let tabElement: HTMLElement;

  /**
   * Initializes Pressure.js on the tab element for pressure-sensitive animations
   * Uses GSAP for smooth, flicker-free animations that respond to high-frequency pressure updates
   */
  onMount(async () => {
    // Dynamically import Pressure.js
    const Pressure = (await import("pressure")).default;

    if (tabElement) {
      Pressure.set(
        tabElement,
        {
          // Called continuously as pressure changes
          change: (force: number) => {
            // Map pressure (0-1) to zoomOut (0-1) with capping
            const zoomValue = Math.min(force * 2, 1);

            // Update state directly - GSAP will handle smoothing in TabContent
            animation.zoomOut = zoomValue;
          },
          // Called when pressure ends - trigger the tab switch
          end: () => {
            // Animate zoomOut back to 0 smoothly with GSAP
            gsap.to(animation, {
              zoomOut: 0,
              duration: 0.3,
              ease: "power2.out",
            });
            onSwitch();
          },
        },
        {
          // Polyfill for devices without pressure support
          polyfill: true,
          polyfillSpeedUp: 300,
          polyfillSpeedDown: 100,
        }
      );
    }
  });
</script>

<div
  bind:this={tabElement}
  onpointerup={(e) => {
    animation.zoomOut = 0;
  }}
  data-tab-id={tab.id}
  class="group relative flex items-center px-3 transition-all duration-200 cursor-pointer select-none w-36 {isActive
    ? 'rounded-t-lg bg-tab-accent shadow-sm'
    : 'rounded-lg bg-tab hover:bg-tab-accent/50'}"
>
  <tab.icon size={14} class="flex-shrink-0 opacity-70 mr-2" />
  <span class="text-xs font-medium truncate flex-1">{tab.title}</span>

  {#if tab.closable}
    <button
      onclick={(e) => {
        e.stopPropagation();
        onClose();
      }}
      class="opacity-0 group-hover:opacity-100 hover:bg-red-500/20 rounded p-0.5 transition-all duration-150 ml-1"
    >
      <X size={12} />
    </button>
  {/if}
</div>
