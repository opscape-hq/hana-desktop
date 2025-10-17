<script lang="ts">
  import { Terminal } from "@xterm/xterm";

  let terminal: HTMLDivElement;

  $effect(() => {
    let term = new Terminal({
      cursorBlink: true,
      fontFamily: "'JetBrains Mono', monospace",
      fontSize: 14,
    });
    term.open(terminal); // open the element bound to the var
    // term.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");

    term.write("aaa")

    // term.onData((data) => {
      
    //   // echo it back so you see it
    //   term.write(data);

    //   // or send it to your backend shell/websocket/etc
    // });
  });
</script>

<div bind:this={terminal}></div>
