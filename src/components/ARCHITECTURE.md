# Tab System Architecture

## Component Hierarchy

```
+layout.svelte (Orchestrator)
│
├── TabBar.svelte (Tab Container)
│   │
│   ├── TabButton.svelte (Tab 1) ← Pressure.js
│   ├── TabButton.svelte (Tab 2) ← Pressure.js
│   ├── TabButton.svelte (Tab N) ← Pressure.js
│   ├── Blue Indicator Bar (Animated)
│   └── Add Tab Button
│
└── TabContent.svelte (Content Container)
    │
    ├── Welcome Tab (Static)
    │   └── +page.svelte content
    │
    └── Terminal Tabs (Dynamic)
        └── TerminalTab.svelte
            ├── Host List View
            └── Terminal View
```

## Data Flow

```
User Interaction
      ↓
TabButton (Pressure.js detects force)
      ↓
pressureScale updates (1.0 → 0.92)
      ↓
Visual zoom-out effect
      ↓
User releases (end callback)
      ↓
onSwitch() callback
      ↓
+layout.svelte switchTab()
      ↓
isTransitioning = true
      ↓
150ms delay (visual feedback)
      ↓
activeTabId updates
      ↓
TabContent slides to new tab
      ↓
Blue indicator bar moves
      ↓
isTransitioning = false
```

## State Management

### Layout State (+layout.svelte)
```typescript
tabs: Tab[]              // Array of all tabs
activeTabId: string      // Currently active tab ID
isTransitioning: boolean // Transition in progress
```

### Tab State (Tab interface)
```typescript
id: string               // Unique identifier
title: string            // Display name
icon: Component          // Lucide icon component
type: 'static' | 'dynamic'
closable: boolean
internalState?: {        // For dynamic tabs
  currentView: string
  connectedHost: string | null
  isConnected: boolean
}
```

### TabButton State
```typescript
pressureScale: number    // 1.0 to 0.92 based on pressure
isPressed: boolean       // Currently being pressed
```

## Animation Timeline

```
Time: 0ms
├─ User presses tab
├─ Pressure.js start() callback
└─ isPressed = true

Time: 0-300ms (variable based on pressure)
├─ Pressure.js change() callback fires continuously
├─ pressureScale updates in real-time
└─ Tab visually zooms out

Time: 300ms (or when user releases)
├─ Pressure.js end() callback
├─ isPressed = false
├─ pressureScale = 1.0
└─ onSwitch() callback fires

Time: 300ms
├─ switchTab() in +layout.svelte
├─ isTransitioning = true
└─ Visual feedback delay starts

Time: 450ms (300 + 150)
├─ activeTabId updates
└─ Content starts sliding

Time: 450-1150ms (700ms transition)
├─ TabContent slides horizontally
├─ 3D perspective effect
├─ Blur effect applied
└─ Blue indicator bar moves

Time: 1150ms
├─ isTransitioning = false
└─ Animation complete
```

## Pressure.js Configuration

```typescript
{
  // Callbacks
  change: (force: number) => void    // 0.0 to 1.0
  start: () => void                  // Pressure begins
  end: () => void                    // Pressure ends
  
  // Options
  polyfill: true                     // Enable fallback
  polyfillSpeedUp: 300              // 300ms to reach full pressure
  polyfillSpeedDown: 100            // 100ms to release
}
```

## CSS Classes & Transitions

### TabButton
```css
transform: scale({pressureScale})
transition: transform 0.1s cubic-bezier(0.4, 0, 0.2, 1)
```

### TabContent
```css
transform: translateX(-{index * 100}%) scale(0.98) rotateY(1deg)
filter: blur(2px)
transition: all 700ms ease-out
```

### Blue Indicator Bar
```css
left: {position}px
width: {width}px
transition: all 300ms ease-out
```

## Event Handlers

### TabButton
- `onSwitch()` - Triggered when tab is released (clicked)
- `onClose()` - Triggered when close button is clicked

### TabBar
- `onTabSwitch(tabId)` - Switch to specific tab
- `onTabClose(tabId)` - Close specific tab
- `onAddTab()` - Create new terminal tab

### TabContent
- `onUpdateTabState(tabId, newState)` - Update tab's internal state

## Performance Considerations

1. **GPU Acceleration**: All animations use `transform` and `filter`
2. **Minimal Repaints**: Only animated properties change
3. **Efficient Updates**: Svelte 5 fine-grained reactivity
4. **Debounced Calculations**: Blue bar position updates with 50ms delay
5. **Hardware Acceleration**: `transform-gpu` class applied

## Accessibility

- Keyboard navigation supported (native button elements)
- ARIA labels can be added for screen readers
- Focus states maintained during transitions
- Close buttons have proper click targets

## Future Enhancements

- [ ] Drag-and-drop tab reordering
- [ ] Tab context menus (right-click)
- [ ] Tab pinning functionality
- [ ] Keyboard shortcuts (Cmd+1, Cmd+2, etc.)
- [ ] Tab groups/categories
- [ ] Persistent tab state (localStorage)
