<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import Editor from './lib/Editor.svelte'


  import CodeMirror from "svelte-codemirror-editor";
  import { python } from "@codemirror/lang-python";

  import io from "socket.io-client";

  import "./app.css";

  const socket = io("/api");

  socket.on("connect", () => {
    console.log("Connected to server");
  });

  socket.emit("join");

  console.log(socket)


  let value = "";
</script>


<main class="flex flex-col justify-start">
  <!-- <h1>Coduels</h1> -->

  <div class="bg-neutral-800 p-3 mb-3 rounded-md">
    <button type="button">Join Room</button>
  </div>

  <div class="bg-neutral-800 p-3 mb-3 rounded-md">
    <h1>h</h1>
  </div>
  <Editor />

</main>
