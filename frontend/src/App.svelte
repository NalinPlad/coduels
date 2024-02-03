<script lang="ts">
  import svelteLogo from './assets/svelte.svg'
  import viteLogo from '/vite.svg'
  import Counter from './lib/Counter.svelte'
  import Editor from './lib/Editor.svelte'


  import CodeMirror from "svelte-codemirror-editor";
  import { python } from "@codemirror/lang-python";

  import "./app.css";


  let value = "";
</script>


<main class="">
  <!-- <h1>Coduels</h1> -->
  
  <div class="card">
    <Editor />
    <!-- <CodeMirror bind:value lang={python()} /> -->
  </div>
</main>
