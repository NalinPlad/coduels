<script>
    import CodeMirror from "svelte-codemirror-editor";
    import { python } from "@codemirror/lang-python";
    import { oneDark } from "@codemirror/theme-one-dark";

    let value="";

    async function run(code) {
        let pyodide = await loadPyodide();
        console.log(pyodide.runPython(code));
    }




</script>

<CodeMirror 
    lang={python()}
    theme={oneDark}
    extensions={[python()]}
    styles={{
        "&" : {
            height: "50vh",
            borderRadius: "10px",
            marginBottom: "10px"
        }
    }}
    bind:value
    placeholder="Write your code here"
/>

<button on:click={run(value)}>
    Run
</button>
