<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import Editor from './lib/Editor.svelte'


  import CodeMirror from "svelte-codemirror-editor";
  import { python } from "@codemirror/lang-python";

  import io from "socket.io-client";

  import "./app.css";

  let socket = io("ws://localhost:3000");

  socket.on("connect", () => {
    console.log("Connected to server");
  });

  socket.on("message-back", (data) => {
    console.log("New message");
    console.log(data);
  });

  // socket.emit("join");
  function startRun() {
    console.log("start")
    socket.emit("start", "Hello")
  }

  function joinRoom() {
    console.log("Joining room")
    socket.emit("join", "")
  }

  function createRoom() {
    console.log("Creating room")
    socket.emit("room", "create")
  }

  console.log(socket)
  let value = "";
</script>


<main class="flex flex-col justify-start">
  <!-- <h1>Coduels</h1> -->

  <div class="bg-neutral-800 p-3 mb-3 rounded-md">
    <button type="button" on:click={joinRoom}>Join Room</button>
    <button type="button" on:click={createRoom}>Create Room</button>
  </div>

  <div class="bg-neutral-800 p-3 mb-3 rounded-md">
    <h1></h1>
  </div>
  <Editor />

</main>
