/**
 * Represents the internal state of a dynamic tab (e.g., terminal)
 */
export interface TabInternalState {
  currentView: "host-list" | "terminal" | "sftp";
  connectedHost: string | null;
  isConnected: boolean;
}

/**
 * Represents a tab in the application
 */
export interface Tab {
  id: string;
  title: string;
  icon: any; // Svelte 5 component type
  type: "static" | "dynamic";
  closable: boolean;
  internalState?: TabInternalState;
}
