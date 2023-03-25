<script lang="ts">
import { invoke } from "@tauri-apps/api";


export let examIdent: string;
export let onQuestionChanged: (newQ: any[]) => void;

const qtypes = ["PredictOutputQuestion" , "MultipleChoiceQuestion" , "WriteCodeQuestion" , "RawQuestion"]

let qtype: string;

let rawQuestion: {latex: string} = {latex: ""}
let multipleChoiceQuestion: {question: string;
    answers: string[]
    correct_id: number,
} = {question: "", answers: [], correct_id: 0}
let _newMCQ: string;

async function addQuestion() {
        let question = null; 
        if(qtype == "RawQuestion") {

question = {qtype, ...rawQuestion}
            } else if (qtype=="MultipleChoiceQuestion") {
question = {qtype, ...multipleChoiceQuestion}

                } else {
                        console.error("panic!")
                    }
            try{
            onQuestionChanged((await invoke('add_question', {
                    examIdent,
                    question
                })))
            } catch(e) {console.error(e)}
    }

</script>

<form>
    <select bind:value={qtype}>
        {#each qtypes as qtype}
            <option value={qtype}>
                {qtype}
            </option>
        {/each}
    </select>

    {#if qtype == "RawQuestion"}
        <p><b>Note:</b>Use pure LaTeX to write your question.</p>
        <label for="question">Question:</label>
        <textarea id="question" bind:value={rawQuestion.latex} placeholder="Question"/>
    {:else if qtype == "MultipleChoiceQuestion"}
        <label for="question">Question:</label>
        <textarea id="question" bind:value={multipleChoiceQuestion.question} placeholder="Question"/>
        
        {#each multipleChoiceQuestion.answers as ans,i}
            <input class={`${multipleChoiceQuestion.correct_id == i ? 'bold' : ''}`} bind:value={ans} placeholder="Answer for MCQ"/>
            <button on:click={() => multipleChoiceQuestion.correct_id = i}>Make Correct answer</button>
        {/each}

        <input bind:value={_newMCQ} placeholder="New answer"/>
        <button on:click={() => {
            multipleChoiceQuestion.answers = [...multipleChoiceQuestion.answers, _newMCQ]
        }}>New answer</button>

    {:else}
    <p>unimplemented</p>
    {/if}

    <button
        disabled={qtypes[0] == qtype || qtypes[2] == qtype}
        on:click={addQuestion}>
    Add Question</button>

</form>

<style>
.bold {
        font-weight: bold;
}
</style>
