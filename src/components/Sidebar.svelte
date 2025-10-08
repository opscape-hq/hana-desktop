<script lang="ts">
  import {
    User,
    Settings,
    Vault,
    Key,
    Server,
    FolderOpen,
  } from "@lucide/svelte";
  import Button from "./ui/Button.svelte";

  let {
    activePanel,
    onPanelToggle,
  }: {
    activePanel: string | null;
    onPanelToggle: (panelId: string) => void;
  } = $props();

  const navItems = [
    { id: "hosts", icon: Server, label: "Hosts" },
    { id: "vaults", icon: Vault, label: "Vaults" },
    { id: "credentials", icon: Key, label: "Credentials" },
    { id: "files", icon: FolderOpen, label: "Files" },
    { id: "account", icon: User, label: "Account" },
    { id: "settings", icon: Settings, label: "Settings" },
  ];
</script>

<div class="flex flex-col gap-2 items-center pt-2 px-2">
  {#each navItems as item}
    <Button
      variant="ghost"
      class="w-full bg-green-500 aspect-square !p-0 flex items-center justify-center rounded-lg {activePanel ===
      item.id
        ? 'bg-white/20'
        : ''}"
      onClick={() => onPanelToggle(item.id)}
    >
      {@const Icon = item.icon}
      <Icon size={14} />
    </Button>
  {/each}
</div>
