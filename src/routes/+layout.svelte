<script lang="ts">
  let { children } = $props();
  import "../app.css";

  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import {
    Maximize,
    Minus,
    X,
    Home,
    Terminal as TerminalIcon,
    Settings,
  } from "@lucide/svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  // Import modular components
  import TabBar from "../components/TabBar.svelte";
  import TabContent from "../components/TabContent.svelte";
  import Sidebar from "../components/Sidebar.svelte";
  import SidePanel from "../components/SidePanel.svelte";
  import type { Tab } from "../types/tab";
  import Button from "../components/ui/Button.svelte";

  let isMac: boolean = $state(false);
  const appWindow = getCurrentWindow();

  /**
   * Fetch system platform information on component mount
   * Used to determine if we should show macOS traffic lights or Windows controls
   */
  onMount(async () => {
    isMac = (await invoke("platform")) === "macos";
  });

  // ============================================
  // Sidebar Panel State Management
  // ============================================

  let activePanel = $state<string | null>(null);

  /**
   * Toggles the sidebar panel on/off
   * Clicking the same icon closes the panel
   */
  function togglePanel(panelId: string) {
    if (activePanel === panelId) {
      activePanel = null;
    } else {
      activePanel = panelId;
    }
  }

  function closePanel() {
    activePanel = null;
  }

  // ============================================
  // Tab State Management
  // ============================================

  let activeTabId = $state("welcome");
  let isTransitioning = $state(false);

  /**
   * Initial tabs configuration
   * Each tab maintains its own internal state and navigation
   */
  let tabs = $state<Tab[]>([
    {
      id: "welcome",
      title: "Welcome",
      icon: Home,
      type: "static",
      closable: false,
    },
    {
      id: "terminal-1",
      title: "root@server1",
      icon: TerminalIcon,
      type: "dynamic",
      closable: true,
      internalState: {
        currentView: "host-list",
        connectedHost: null,
        isConnected: false,
      },
    },
  ]);

  /**
   * Switches to a different tab with smooth transition animation
   * Prevents switching if already transitioning or clicking the active tab
   */
  function switchTab(tabId: string) {
    if (tabId === activeTabId || isTransitioning) return;

    isTransitioning = true;

    // Delay the actual tab switch to allow zoom-out animation to complete
    setTimeout(() => {
      activeTabId = tabId;
      setTimeout(() => {
        isTransitioning = false;
      }, 50);
    }, 150);
  }

  /**
   * Closes a tab and switches to another if the closed tab was active
   */
  function closeTab(tabId: string) {
    const tabIndex = tabs.findIndex((tab) => tab.id === tabId);
    if (tabIndex === -1) return;

    tabs = tabs.filter((tab) => tab.id !== tabId);

    // If we closed the active tab, switch to another
    if (activeTabId === tabId && tabs.length > 0) {
      const newActiveIndex = Math.max(0, tabIndex - 1);
      activeTabId = tabs[newActiveIndex].id;
    }
  }

  /**
   * Creates and opens a new terminal tab
   */
  function addNewTerminalTab() {
    const newTabId = `terminal-${Date.now()}`;
    const newTab: Tab = {
      id: newTabId,
      title: "New Terminal",
      icon: TerminalIcon,
      type: "dynamic",
      closable: true,
      internalState: {
        currentView: "host-list",
        connectedHost: null,
        isConnected: false,
      },
    };

    tabs = [...tabs, newTab];
    switchTab(newTabId);
  }

  /**
   * Updates a tab's internal state (e.g., when connecting to a host)
   * Also updates the tab title based on the new state
   */
  function updateTabState(tabId: string, newState: any) {
    const tab = tabs.find((t) => t.id === tabId);
    if (tab && tab.internalState) {
      tab.internalState = { ...tab.internalState, ...newState };

      // Update tab title based on state
      if (newState.connectedHost) {
        tab.title = `${newState.connectedHost}`;
      } else if (newState.currentView === "host-list") {
        tab.title = "Host List";
      }
    }
  }
</script>

<div
  class="antialiased absolute inset-0 flex flex-col h-screen bg-titlebar-background"
>
  <!-- Titlebar -->
  <div class="select-none min-h-10 box-border text-sm flex">
    <!-- Traffic lights space (macOS) or minimal padding (Windows) -->
    <div class={isMac ? "w-[78px]" : "w-6"} data-tauri-drag-region></div>

    <!-- Tab bar with modular TabBar component -->
    <TabBar
      {tabs}
      {activeTabId}
      onTabSwitch={switchTab}
      onTabClose={closeTab}
      onAddTab={addNewTerminalTab}
    />

    <!-- Window controls (Windows only) -->
    {#if !isMac}
      <div class="ml-auto flex">
        <button
          onclick={() => appWindow.minimize()}
          class="h-full w-10 hover:bg-zinc-600/60 transition-colors flex justify-center items-center"
        >
          <Minus size={16} />
        </button>
        <button
          onclick={() => appWindow.toggleMaximize()}
          class="h-full w-10 hover:bg-zinc-600/60 transition-colors flex justify-center items-center"
        >
          <Maximize size={16} />
        </button>
        <button
          onclick={() => appWindow.close()}
          class="h-full w-10 hover:bg-red-600/60 transition-colors flex justify-center items-center"
        >
          <X size={16} />
        </button>
      </div>
    {/if}
  </div>

  <!-- Main content area with spacing and rounded corners -->
  <div class="flex overflow-hidden flex-1">
    <div class="w-14 flex flex-col justify-start">
      <!-- Sidebar Navigation -->
      <Sidebar activePanel={activePanel} onPanelToggle={togglePanel} />
    </div>
    <div class="relative flex-1 overflow-hidden m-1 ml-0 rounded-xl">
      <!-- Side Panel -->
      <SidePanel activePanel={activePanel} onClose={closePanel} />
      
      <!-- Main content with blur effect when panel is open -->
      <div class="h-full {activePanel ? 'blur-sm pointer-events-none' : ''} transition-all duration-300">
        <!-- Tab content with modular TabContent component -->
        <TabContent
          {tabs}
          {activeTabId}
          {isTransitioning}
          onUpdateTabState={updateTabState}
        >
          {@render children()}
        </TabContent>
      </div>
    </div>
  </div>
</div>
