<script lang="ts">
import { goto } from '$app/navigation';
import {dialog, invoke} from "@tauri-apps/api"
import CreateExam from "../lib/CreateExam.svelte";

async function loadExam() {
    const directory = await dialog.open({
        directory: true,
        title: "Choose the folder with your exam",
        multiple: false,
        recursive: false
    });

    if (!directory) {err = "No directory chosen."; return;}

    try{
        const val: [any, string] = await invoke("load_exam", {
            directory,
        })
        goto(`/exam/${val[1]}`)
        err = ""
    } catch(e) {
        err =(e as any)
    }
}

let err = ""

</script>

<h1>Welcome to examsh</h1>

<button on:click={loadExam}>Load existing exam</button>
<p>{err}</p>

<CreateExam />
