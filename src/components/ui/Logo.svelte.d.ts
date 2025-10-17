import { SvelteComponentTyped } from 'svelte';

type Variant = 'dark' | 'light';

// When raster = true, only `variant` and `class` (or className) are allowed.
// When raster = false, only `foreground`, `background` and `class` are allowed.
export type LogoProps =
  | ({ raster: true; variant: Variant; class?: string; className?: string } & Record<string, any>)
  | ({ raster?: false; foreground?: string; background?: string; class?: string; className?: string } & Record<string, any>);

export default class Logo extends SvelteComponentTyped<LogoProps> {}
