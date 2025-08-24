<script>
  import Logo from "../components/Logo.svelte";
</script>

<Logo class="absolute w-lg left-1/2 -translate-x-1/2 z-10" foreground="black" />
<div class="animate-in blur-in fade-in duration-5000 blur-2xl">
  <div
    class="absolute w-lg left-1/2 -translate-x-1/2 -z-20 mx-auto aspect-square bg-gradient-to-r from-rose-500 via-indigo-600 to-emerald-300"
    style="mask: url('/Logo Dark.png'); mask-repeat: no-repeat; mask-position: center; mask-size: contain;mask-mode:luminance;"
  ></div>
</div>

<main class="relative h-screen flex flex-col items-center justify-center z-20">
  <h1>Welcome</h1>
  <p>Let's get you started</p>
</main>
