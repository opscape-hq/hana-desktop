<script lang="ts">
  import Button from "../components/ui/Button.svelte";

  let isLoading = $state(true);

  /**
   * Simulates initial loading with a smooth zoom-out effect
   * Creates a polished entrance animation for the welcome page
   */
  $effect(() => {
    setTimeout(() => {
      isLoading = false;
    }, 1200);
  });
</script>

<!-- Main container with Windows 11-style zoom out effect -->
<div class="relative h-full overflow-hidden ">

  <!-- Content with zoom out + blur effect -->
  <div
    class="relative h-full transition-all duration-1000 ease-out {
      isLoading ? 'scale-110 blur-md opacity-0' : 'scale-100 blur-0 opacity-100'
    }"
  >
    <div class="h-full flex flex-col items-center justify-center p-8">
      <div class="text-center space-y-8 max-w-md">
        <div class="space-y-4">
          <h1 class="font-light text-4xl text-white/90 tracking-wide">
            Simple Page
          </h1>
          <p class="text-white/60 text-lg leading-relaxed">
            Clean and minimal content goes here.
          </p>
        </div>
        
        <div class="flex flex-col gap-4">
          <Button>Primary Action</Button>
          
          <Button variant="secondary">Secondary Action</Button>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  @keyframes pulse-slow {
    0%, 100% { opacity: 0.3; transform: scale(1); }
    50% { opacity: 0.6; transform: scale(1.05); }
  }
  
  @keyframes pulse-slow-delayed {
    0%, 100% { opacity: 0.2; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(1.08); }
  }
  
  .animate-pulse-slow {
    animation: pulse-slow 8s ease-in-out infinite;
  }
  
  .animate-pulse-slow-delayed {
    animation: pulse-slow-delayed 10s ease-in-out infinite 2s;
  }
</style>