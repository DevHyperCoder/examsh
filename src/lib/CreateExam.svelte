<script lang="ts">
import {dialog, invoke} from "@tauri-apps/api"
async function createNewExam() {
    const directory = await dialog.open({
            directory: true,
            title: "Choose a folder to store your new exam",
            multiple: false,
            recursive: false
        });


    if (!directory) {str = "No directory chosen."; return;}
    try{
        await invoke("create_new_exam", {
            directory,
            examSchema: exam_schema
        })
        str = ""
    } catch(e) {
            str =(e as any)
        }
}

let exam_schema: {
    course_name: string,
    test_name: string,
    instructions: string,
    marking_instructions: string,
    } = {
            course_name: "",
            test_name: "Example test",
            instructions: "Read all questions carefully",
            marking_instructions: "Mark properly",
        }
let str = ""
</script>

<h1> Create new exam </h1>

<form>

<div>
<label for="course-name">Course name: </label>
<input id="course-name" placeholder="Enter new course name: " bind:value={exam_schema.course_name} />
</div>

<div>
<label for="test-name">Test name: </label>
<input id="test-name" placeholder="Enter new test name: " bind:value={exam_schema.test_name} />
</div>

<div>
<label for="instructions">Test Instructions: </label>
<textarea id="instructions" placeholder="Test instructions" bind:value={exam_schema.instructions}/>
</div>

<div>
<label for="marking-instructions">Marking Instructions: </label>
<textarea id="marking-instructions" placeholder="Marking instructions" bind:value={exam_schema.marking_instructions}/>
</div>

<button on:click={createNewExam}>Create new exam</button>
</form>

<p>{str}</p>
<style>
div {
        display: flex;
        flex-direction: column;
        gap: .25rem;
    }
form {
        display: flex;
        flex-direction: column;
        width:60%;
        margin: 0 auto;
        gap: 1rem;
    }
</style>
