<script lang="ts">
import {dialog, invoke} from "@tauri-apps/api"
import CreateExam from "../lib/CreateExam.svelte";

async function loadExam() {
    const directory = await dialog.open({
            directory: true,
            title: "Choose the folder with your exam",
            multiple: false,
            recursive: false
        });


    if (!directory) {str = "No directory chosen."; return;}
    try{
        val = await invoke("load_exam", {
            directory,
        })
        str = ""
    } catch(e) {
            str =(e as any)
            val = ""
        }
}

let str = ""
let val:any = []

</script>

<h1>Welcome to examsh</h1>
<a href="/index"> index</a>

<a href="/exam/{val[1] || '404'}">Go to exam</a>

<button on:click={loadExam}>Load existing exam</button>
<pre>
<code>
{val}
</code>
</pre>

<p>{str}</p>

<CreateExam />
