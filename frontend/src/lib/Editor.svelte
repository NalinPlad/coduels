<script lang="ts">
    import CodeMirror from "svelte-codemirror-editor";
    import { python } from "@codemirror/lang-python";
    import { oneDark } from "@codemirror/theme-one-dark";

    let value="";

    enum OutputType {
        STDOUT = 0,
        STDERR = 1,
        NOTE = 2,
    }

    interface Output {
        type: OutputType,
        data: string,
    }

    let console_output: Output[] = [];

    async function initPyodide() {
        let pyodide = await loadPyodide({
            stdout: (msg) => {
                console_output.push({
                    type: OutputType.STDOUT,
                    data: msg
                });
                console_output = console_output;
            },
        });

        console_output.push({
            type: OutputType.NOTE,
            data: "Python is ready"
        });
        console_output = console_output;

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

<div class="flex flex-row justify-evenly h-screen max-h-[40vh] gap-3 bg-neutral-800 rounded-md p-3 mb-2">
    <CodeMirror 
        lang={python()}
        theme={oneDark}
        extensions={[python()]}
        styles={{
            "&" : {
                height: "100%",
            }
        }}

        class="w-3/4"
        
        bind:value
        placeholder="Write your code here"
    />
    <div class="w-1/4">
        {#each console_output as message}
            {#if message.type === OutputType.STDOUT}
                <p class="border-b-2 bg-neutral-700 p-1 text-sm">{message.data}</p>
            {/if}
            {#if message.type === OutputType.STDERR}
                <p class="border-b-2 bg-neutral-700 p-1 text-sm">{message.data}</p>
            {/if}
            {#if message.type === OutputType.NOTE}
                <p class="border-b-2 bg-neutral-700 p-1 text-sm"><i>[NOTE]</i> {message.data}</p>
            {/if}
        {/each}
    </div>
</div>
    

<button on:click={run(value)}>
    Run
</button>
