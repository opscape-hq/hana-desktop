# Migration Guide: Monolithic to Modular Tab System

## Overview
This guide explains the changes made to refactor the tab system from a monolithic structure to a modular, maintainable architecture with pressure-sensitive animations.

## File Changes

### New Files Created
```
✅ src/components/TabButton.svelte      - Individual tab component
✅ src/components/TabBar.svelte          - Tab container component
✅ src/components/TabContent.svelte      - Content area component
✅ src/components/TerminalTab.svelte     - Terminal tab content
✅ src/components/README.md              - Component documentation
✅ src/components/ARCHITECTURE.md        - Architecture overview
✅ src/types/tab.ts                      - TypeScript interfaces
✅ src/types/pressure.d.ts               - Pressure.js types
✅ REFACTORING_SUMMARY.md                - Summary of changes
✅ MIGRATION_GUIDE.md                    - This file
```

### Modified Files
```
📝 src/routes/+layout.svelte             - Simplified to use components
📝 src/routes/+page.svelte               - Added TypeScript and comments
```

## Breaking Changes

### None! 
The refactoring maintains 100% backward compatibility. All existing functionality works exactly as before, but with:
- Better code organization
- Pressure-sensitive animations
- Improved spacing and rounding
- Full TypeScript support
- Comprehensive documentation

## New Features

### 1. Pressure-Sensitive Tab Interactions
Tabs now respond to pressure/force on supported devices:
- **macOS**: Force Touch on trackpads
- **iOS**: 3D Touch on compatible devices
- **Fallback**: Time-based polyfill for other devices

### 2. Enhanced Visual Feedback
- Smooth zoom-out effect when pressing tabs
- 3D perspective during transitions
- Blur effect for depth perception
- Proper spacing between tabs (8px gap)

### 3. Modular Architecture
Each component has a single, clear responsibility:
- Easy to test individual components
- Simple to add new tab types
- Clear data flow and state management

## How to Use

### Adding a New Tab Type

1. Create a new component in `src/components/`:
```svelte
<!-- src/components/MyNewTab.svelte -->
<script lang="ts">
  import type { Tab } from "../types/tab";
  
  interface Props {
    tab: Tab;
    onUpdateTabState: (tabId: string, newState: any) => void;
  }
  
  let { tab, onUpdateTabState }: Props = $props();
</script>

<div class="h-full">
  <!-- Your tab content here -->
</div>
```

2. Import and use in `TabContent.svelte`:
```svelte
import MyNewTab from "./MyNewTab.svelte";

<!-- In the template -->
{:else if tab.type === 'my-new-type'}
  <MyNewTab {tab} {onUpdateTabState} />
{/if}
```

3. Create tabs with the new type in `+layout.svelte`:
```typescript
{
  id: 'my-tab-1',
  title: 'My Tab',
  icon: MyIcon,
  type: 'my-new-type',
  closable: true,
  internalState: { /* your state */ }
}
```

### Customizing Animations

#### Adjust Pressure Sensitivity
In `TabButton.svelte`, modify the scale calculation:
```typescript
// Current: 8% zoom out (0.92 scale)
pressureScale = 1 - force * 0.08;

// More dramatic: 15% zoom out (0.85 scale)
pressureScale = 1 - force * 0.15;

// Subtle: 4% zoom out (0.96 scale)
pressureScale = 1 - force * 0.04;
```

#### Adjust Transition Speed
In `TabContent.svelte`, modify the duration:
```css
/* Current: 700ms */
transition-all duration-700

/* Faster: 400ms */
transition-all duration-400

/* Slower: 1000ms */
transition-all duration-1000
```

#### Adjust Polyfill Speed
In `TabButton.svelte`, modify the config:
```typescript
{
  polyfillSpeedUp: 300,    // Time to reach full pressure
  polyfillSpeedDown: 100,  // Time to release
}
```

### Customizing Spacing

In `+layout.svelte`, adjust the margins:
```svelte
<!-- Current: 8px margin, 12px rounded corners -->
<div class="... m-2 rounded-xl">

<!-- Larger spacing: 16px margin, 16px rounded corners -->
<div class="... m-4 rounded-2xl">

<!-- Minimal spacing: 4px margin, 8px rounded corners -->
<div class="... m-1 rounded-lg">
```

In `TabBar.svelte`, adjust tab spacing:
```svelte
<!-- Current: 8px gap -->
<div class="... gap-2">

<!-- Larger gap: 16px -->
<div class="... gap-4">

<!-- Minimal gap: 4px -->
<div class="... gap-1">
```

## Testing Checklist

After migration, test these scenarios:

- [ ] Click tabs to switch between them
- [ ] Close closable tabs
- [ ] Add new terminal tabs
- [ ] Connect to hosts in terminal tabs
- [ ] Disconnect from hosts
- [ ] Test on macOS with Force Touch trackpad
- [ ] Test on iOS with 3D Touch
- [ ] Test with regular mouse (polyfill)
- [ ] Test with 5+ tabs open
- [ ] Test rapid tab switching
- [ ] Verify no TypeScript errors
- [ ] Check browser console for errors

## Troubleshooting

### Pressure.js Not Working
1. Check that `pressure` is installed: `npm list pressure`
2. Verify type definitions exist: `src/types/pressure.d.ts`
3. Check browser console for import errors
4. Test polyfill by using a regular mouse

### Animations Stuttering
1. Ensure GPU acceleration is enabled in browser
2. Check for other heavy processes
3. Reduce transition duration for testing
4. Verify `transform-gpu` class is applied

### TypeScript Errors
1. Run `npm run check` to see all errors
2. Verify all type files are in place
3. Check that imports are correct
4. Restart TypeScript server in IDE

### Tabs Not Switching
1. Check browser console for errors
2. Verify `activeTabId` is updating
3. Check that `switchTab()` is being called
4. Ensure `isTransitioning` is resetting

## Rollback Instructions

If you need to rollback to the original monolithic structure:

1. Restore the original `+layout.svelte` from git:
```bash
git checkout HEAD~1 src/routes/+layout.svelte
```

2. Remove new files:
```bash
rm -rf src/components/Tab*.svelte
rm -rf src/types/tab.ts src/types/pressure.d.ts
rm -rf src/components/README.md src/components/ARCHITECTURE.md
```

3. Restart dev server

## Support

For questions or issues:
1. Check `src/components/README.md` for component documentation
2. Review `src/components/ARCHITECTURE.md` for architecture details
3. Read `REFACTORING_SUMMARY.md` for feature overview
4. Check inline code comments for implementation details

## Next Steps

Consider these enhancements:
1. Add keyboard shortcuts (Cmd+1, Cmd+2, etc.)
2. Implement drag-and-drop tab reordering
3. Add tab context menus
4. Persist tab state to localStorage
5. Add tab groups/categories
6. Implement tab search/filter
