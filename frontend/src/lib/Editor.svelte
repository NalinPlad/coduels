<script>
    import CodeMirror from "svelte-codemirror-editor";
    import { python } from "@codemirror/lang-python";
    import { oneDark } from "@codemirror/theme-one-dark";

    let value="print('Hello World!')";
</script>

<CodeMirror/>
