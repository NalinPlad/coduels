<script>
    import CodeMirror from "svelte-codemirror-editor";
    import { python } from "@codemirror/lang-python";
    import { oneDark } from "@codemirror/theme-one-dark";

    let value="";
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

<button>
    Run
</button>
