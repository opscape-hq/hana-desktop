<script lang="ts">
  import type { Tab } from "../types/tab";

  interface Props {
    tab: Tab;
    onUpdateTabState: (tabId: string, newState: any) => void;
  }

  let { tab, onUpdateTabState }: Props = $props();

  /**
   * Handles connecting to a host
   * Updates the tab's internal state to show the terminal view
   */
  function connectToHost(host: string) {
    onUpdateTabState(tab.id, {
      currentView: "terminal",
      connectedHost: host,
      isConnected: true,
    });
  }

  /**
   * Handles disconnecting from the current host
   * Returns to the host list view
   */
  function disconnect() {
    onUpdateTabState(tab.id, {
      currentView: "host-list",
      connectedHost: null,
      isConnected: false,
    });
  }
</script>

<div class="h-full">
  {#if tab.internalState?.currentView === "host-list"}
    <!-- Host list view -->
    <div
      class="h-full flex flex-col items-center justify-center p-8"
    >
      <div class="text-center space-y-6 max-w-md">
        <div class="space-y-2">
          <h2 class="text-2xl font-light text-white/90">Select Host</h2>
          <p class="text-white/60">Choose a server to connect to</p>
        </div>

        <div class="space-y-3">
          {#each ["server1.example.com", "server2.example.com", "localhost"] as host}
            <button
              onclick={() => connectToHost(host)}
              class="w-full p-4 bg-white/5 backdrop-blur-sm border border-white/20 rounded-xl text-white/80 font-medium transition-all duration-300 hover:bg-white/10 hover:scale-105 text-left"
            >
              <div class="flex items-center gap-3">
                <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                <span>{host}</span>
              </div>
            </button>
          {/each}
        </div>
      </div>
    </div>
  {:else if tab.internalState?.currentView === "terminal"}
    <!-- Terminal view -->
    <div class="h-full flex flex-col bg-slate-950">
      <!-- Terminal header -->
      <div
        class="flex items-center justify-between p-4 bg-slate-900/50 border-b border-slate-700/50"
      >
        <div class="flex items-center gap-3">
          <div class="w-3 h-3 bg-green-400 rounded-full animate-pulse"></div>
          <span class="text-white/90 font-medium"
            >Connected to {tab.internalState.connectedHost}</span
          >
        </div>
        <button
          onclick={disconnect}
          class="px-3 py-1 bg-red-500/20 hover:bg-red-500/30 text-red-300 rounded text-sm transition-colors"
        >
          Disconnect
        </button>
      </div>

      <!-- Terminal content -->
      <div
        class="flex-1 p-4 font-mono text-sm text-green-400 bg-black/20"
      >
        <div class="space-y-1">
          <div>user@{tab.internalState.connectedHost}:~$ neofetch</div>
          <div class="text-blue-400">                   -`</div>
          <div class="text-blue-400">                  .o+`</div>
          <div class="text-blue-400">                 `ooo/</div>
          <div class="text-blue-400">                `+oooo:</div>
          <div class="text-blue-400">               `+oooooo:</div>
          <div class="text-blue-400">               -+oooooo+:</div>
          <div class="text-blue-400">             `/:-:++oooo+:</div>
          <div class="text-white/60">
            user@{tab.internalState.connectedHost}:~$ █
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
