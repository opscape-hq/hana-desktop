<script lang="ts">
  const variants = {
    primary: "bg-primary text-primary-foreground hover:bg-primary/80 border border-primary",
    secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80 border border-secondary",
  };

  let {
    children,
    onClick = (event: MouseEvent ) => {},
    variant = "primary",
    class: className = "",
  }: {
    children: any;
    onClick?: (event: MouseEvent) => void;
    variant?: keyof typeof variants;
    class?: string;
  } = $props();
</script>

<button class={`py-1 px-4 rounded-sm transition-colors ${variants[variant]} ${className}`} onclick={onClick}>
  {@render children()}
</button>