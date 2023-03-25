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

    let predictOutputQuestion: {
        question: string;
        pre_run: string;
        run: string;
        post_run: string;
        _code: string[][];
    } = {
        question: "",
        pre_run: "",
        run:"",
        post_run: "",
        _code: []
    }

async function addQuestion() {
        let question = null; 
        if(qtype == "RawQuestion") {

question = {qtype, ...rawQuestion}
            } else if (qtype=="MultipleChoiceQuestion") {
question = {qtype, ...multipleChoiceQuestion}

                } else {

question = {qtype, ...predictOutputQuestion}
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
    {:else if qtype == "PredictOutputQuestion"}
        <label for="question">Question:</label>
        <textarea id="question" bind:value={predictOutputQuestion.question} placeholder="Question"/>

        <label for="pre-run">Command to run BEFORE execution (compilation etc)</label>
        <input id="pre-run" bind:value={predictOutputQuestion.pre_run} />

        <label for="run">Command to run FOR execution (actual running of the program)</label>
        <input id="run" bind:value={predictOutputQuestion.run} />

        <label for="post-run">Command to run AFTER execution (cleanup etc)</label>
        <input id="post-run" bind:value={predictOutputQuestion.post_run} />

        {#each predictOutputQuestion._code as code_file}
            <label for="fname">File Name:</label>
            <input id="fname" bind:value={code_file[0]} placeholder="Filename"/>
            <label for="code">Code:</label>
            <textarea id="code" bind:value={code_file[1]} placeholder="Code"/>
        {/each}

        <button on:click={() => predictOutputQuestion._code = [...predictOutputQuestion._code, ["", ""]]}>create new</button>

    {:else}
    <p>unimplemented</p>
    {/if}

    <button
        disabled={qtypes[2] == qtype}
        on:click={addQuestion}>
    Add Question</button>

</form>

<style>
.bold {
        font-weight: bold;
}
</style>
