# Tab System Components

This directory contains the modular components for the tab system with pressure-sensitive animations.

## Component Structure

### TabButton.svelte
Individual tab button with pressure-sensitive animations using Pressure.js.

**Features:**
- Pressure-sensitive zoom-out effect when pressing down
- Smooth scale transitions (1.0 → 0.92 based on pressure)
- Polyfill support for devices without pressure sensitivity
- Close button for closable tabs

**Props:**
- `tab`: Tab object containing id, title, icon, etc.
- `isActive`: Boolean indicating if this tab is currently active
- `onSwitch`: Callback function triggered when tab is released (clicked)
- `onClose`: Callback function triggered when close button is clicked

### TabBar.svelte
Container for all tabs with the animated blue indicator bar.

**Features:**
- Manages tab layout and spacing
- Animated blue indicator bar that follows the active tab
- Add new tab button
- Responsive positioning calculations

**Props:**
- `tabs`: Array of all tab objects
- `activeTabId`: ID of the currently active tab
- `onTabSwitch`: Callback for switching tabs
- `onTabClose`: Callback for closing tabs
- `onAddTab`: Callback for adding new tabs

### TabContent.svelte
Container for tab content with smooth sliding transitions.

**Features:**
- Horizontal sliding animation between tabs
- 3D perspective effects during transitions
- Blur effect during transitions for smooth visual feedback
- Renders different content based on tab type

**Props:**
- `tabs`: Array of all tab objects
- `activeTabId`: ID of the currently active tab
- `isTransitioning`: Boolean indicating if a transition is in progress
- `onUpdateTabState`: Callback for updating tab internal state
- `children`: Slot for rendering the welcome page content

### TerminalTab.svelte
Handles the dynamic content for terminal tabs.

**Features:**
- Host selection view
- Terminal view with connection status
- Disconnect functionality
- State management for connected hosts

**Props:**
- `tab`: The terminal tab object
- `onUpdateTabState`: Callback for updating the tab's internal state

## Animation Details

### Pressure-Sensitive Tab Press
- Uses Pressure.js to detect force/pressure on trackpads and touch devices
- Maps pressure values (0-1) to scale values (1.0 to 0.92)
- Smooth cubic-bezier easing: `cubic-bezier(0.4, 0, 0.2, 1)`
- Polyfill enabled for devices without pressure support (time-based fallback)

### Tab Switching Transition
1. User presses tab → zoom-out animation starts
2. User releases → `onSwitch` callback fires
3. 150ms delay for visual feedback
4. Content slides horizontally with 700ms duration
5. 3D perspective and blur effects during transition
6. Blue indicator bar smoothly moves to new position

### Spacing and Rounding
- Tabs have `gap-2` (8px) spacing between them
- Each tab has `rounded-lg` corners
- Content area has `m-2` margin and `rounded-xl` corners
- Blue indicator bar has `rounded-full` for smooth appearance

## Type Definitions

See `src/types/tab.ts` for TypeScript interfaces:
- `Tab`: Main tab interface
- `TabInternalState`: State for dynamic tabs (terminal, SFTP, etc.)

See `src/types/pressure.d.ts` for Pressure.js type definitions.

## Usage Example

```svelte
<script lang="ts">
  import TabBar from "../components/TabBar.svelte";
  import TabContent from "../components/TabContent.svelte";
  import type { Tab } from "../types/tab";

  let tabs = $state<Tab[]>([...]);
  let activeTabId = $state("welcome");
  let isTransitioning = $state(false);

  function switchTab(tabId: string) {
    if (tabId === activeTabId || isTransitioning) return;
    isTransitioning = true;
    setTimeout(() => {
      activeTabId = tabId;
      setTimeout(() => isTransitioning = false, 50);
    }, 150);
  }
</script>

<TabBar
  {tabs}
  {activeTabId}
  onTabSwitch={switchTab}
  onTabClose={closeTab}
  onAddTab={addNewTab}
/>

<TabContent
  {tabs}
  {activeTabId}
  {isTransitioning}
  onUpdateTabState={updateTabState}
>
  <!-- Welcome page content -->
</TabContent>
```

## Performance Notes

- All animations use `transform` and `filter` for GPU acceleration
- `transform-gpu` class ensures hardware acceleration
- Transitions use optimized easing functions
- Pressure.js polyfill speeds are tuned for smooth fallback (300ms up, 100ms down)
