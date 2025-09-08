<script>
  import Button from "../components/ui/Button.svelte";
  import Logo from "../components/ui/Logo.svelte";
</script>

<main class="h-full flex flex-col items-center justify-center z-20 gap-4">
  <h1 class="font-light text-3xl">Setup</h1>
  <p>Signing in allows you to sync your data between devices securely.</p>
  <div class="flex gap-4">
    <Button variant="primary">Log In</Button>
    <Button variant="secondary">Sign up</Button>
  </div>
</main>

<Logo class="w-14 h-14 absolute bottom-4 left-1/2 -translate-x-1/2" />