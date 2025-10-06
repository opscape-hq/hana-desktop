/**
 * Type definitions for Pressure.js
 * Pressure.js is a library for handling pressure-sensitive touch and click events
 */

declare module "pressure" {
  interface PressureConfig {
    polyfill?: boolean;
    polyfillSpeedUp?: number;
    polyfillSpeedDown?: number;
    only?: "touch" | "mouse" | "pointer";
    preventSelect?: boolean;
  }

  interface PressureCallbacks {
    start?: () => void;
    end?: () => void;
    startDeepPress?: () => void;
    endDeepPress?: () => void;
    change?: (force: number, event: Event) => void;
    unsupported?: () => void;
  }

  interface PressureStatic {
    set(
      element: HTMLElement | string,
      callbacks: PressureCallbacks,
      config?: PressureConfig
    ): void;
    config(config: PressureConfig): void;
    map(
      inputValue: number,
      inputMin: number,
      inputMax: number,
      outputMin: number,
      outputMax: number
    ): number;
  }

  const Pressure: PressureStatic;
  export default Pressure;
}
