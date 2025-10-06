/**
 * Global animation state for tab interactions
 * 
 * @property zoomOut - Pressure-based zoom level (0-1)
 *   - 0 = no zoom (normal size)
 *   - 1 = max zoom out (80% scale)
 *   - GSAP handles smooth interpolation, not CSS transitions
 */
export const animation = $state({ zoomOut: 0 });
