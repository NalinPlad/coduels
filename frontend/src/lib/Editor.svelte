<script>
    import CodeMirror from "svelte-codemirror-editor";
    import { python } from "@codemirror/lang-python";
    import { oneDark } from "@codemirror/theme-one-dark";

    let value="";
    let console_output=[];

    async function initPyodide() {
        let pyodide = await loadPyodide({
            stdout: (msg) => {
                console_output.push(msg);
                console_output = console_output;
            },
        });

        console_output.push("Loaded Python wasm module");

        return pyodide;
    }

    let pyodidePromise = initPyodide();

    async function run(code) {
        let pyodide = await pyodidePromise;

        try {
            pyodide.runPython(code);
        } catch (error) {
            console.log(error);
        }

        console.log(console_output)
    }






</script>

<div>
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
    <div>
        {#each console_output as message}
        <p>{message}</p>
        {/each}
    </div>
</div>
    

<button on:click={run(value)}>
    Run
</button>
