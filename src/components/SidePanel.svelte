<script lang="ts">
  import { X } from "@lucide/svelte";
  import Button from "./ui/Button.svelte";

  let {
    activePanel,
    onClose,
  }: {
    activePanel: string | null;
    onClose: () => void;
  } = $props();

  const panelTitles: Record<string, string> = {
    hosts: "Hosts",
    vaults: "Vaults",
    credentials: "Credentials",
    files: "Files",
    account: "Account",
    settings: "Settings",
  };
</script>

  <div
    class="absolute left-0 top-0 bottom-0 w-80 bg-titlebar-background backdrop-blur-sm border-r border-zinc-700 z-10 shadow-2xl {activePanel ? 'translate-x-0' : '-translate-x-full'} transition-transform duration-300"
  >
    <div class="flex items-center justify-between p-4 border-b border-zinc-700">
      <h2 class="text-lg font-semibold">{panelTitles[activePanel] || activePanel}</h2>
      <Button variant="ghost" class="w-8 h-8 p-0 flex items-center justify-center" onClick={onClose}>
        <X size={20} />
      </Button>
    </div>
    
    <div class="p-4 overflow-y-auto" style="height: calc(100% - 4rem);">
      {#if activePanel === "hosts"}
        <div class="text-zinc-400">
          <p class="mb-2">Manage your SSH hosts</p>
          <div class="space-y-2">
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">server1.example.com</div>
              <div class="text-sm text-zinc-500">root@192.168.1.100</div>
            </div>
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">server2.example.com</div>
              <div class="text-sm text-zinc-500">admin@192.168.1.101</div>
            </div>
          </div>
        </div>
      {:else if activePanel === "vaults"}
        <div class="text-zinc-400">
          <p class="mb-2">Secure credential vaults</p>
          <div class="space-y-2">
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">Production Vault</div>
              <div class="text-sm text-zinc-500">12 credentials</div>
            </div>
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">Development Vault</div>
              <div class="text-sm text-zinc-500">8 credentials</div>
            </div>
          </div>
        </div>
      {:else if activePanel === "credentials"}
        <div class="text-zinc-400">
          <p class="mb-2">Saved SSH credentials</p>
          <div class="space-y-2">
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">root</div>
              <div class="text-sm text-zinc-500">SSH Key Authentication</div>
            </div>
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">admin</div>
              <div class="text-sm text-zinc-500">Password Authentication</div>
            </div>
          </div>
        </div>
      {:else if activePanel === "files"}
        <div class="text-zinc-400">
          <p class="mb-2">File transfer and management</p>
          <div class="space-y-2">
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">Recent Transfers</div>
              <div class="text-sm text-zinc-500">View transfer history</div>
            </div>
            <div class="p-3 bg-zinc-800 rounded-lg hover:bg-zinc-700 cursor-pointer transition-colors">
              <div class="font-medium">Bookmarked Paths</div>
              <div class="text-sm text-zinc-500">Quick access locations</div>
            </div>
          </div>
        </div>
      {:else if activePanel === "account"}
        <div class="text-zinc-400">
          <p class="mb-2">Account information</p>
          <div class="space-y-4">
            <div class="p-3 bg-zinc-800 rounded-lg">
              <div class="text-sm text-zinc-500 mb-1">Username</div>
              <div class="font-medium">user@example.com</div>
            </div>
            <div class="p-3 bg-zinc-800 rounded-lg">
              <div class="text-sm text-zinc-500 mb-1">Plan</div>
              <div class="font-medium">Professional</div>
            </div>
          </div>
        </div>
      {:else if activePanel === "settings"}
        <div class="text-zinc-400">
          <p class="mb-2">Application settings</p>
          <div class="space-y-3">
            <div>
              <div class="font-medium mb-2">Appearance</div>
              <div class="p-3 bg-zinc-800 rounded-lg">
                <label class="flex items-center justify-between cursor-pointer">
                  <span class="text-sm">Dark mode</span>
                  <input type="checkbox" checked class="w-4 h-4" />
                </label>
              </div>
            </div>
            <div>
              <div class="font-medium mb-2">Terminal</div>
              <div class="p-3 bg-zinc-800 rounded-lg">
                <label class="flex items-center justify-between cursor-pointer">
                  <span class="text-sm">Font size</span>
                  <input type="number" value="14" class="w-16 bg-zinc-700 px-2 py-1 rounded text-sm" />
                </label>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>