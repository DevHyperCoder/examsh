<script lang="ts">
  import Badge from "$lib/Badge.svelte";
  import Button from "$lib/Button.svelte";
  import Input from "$lib/Input.svelte";

  import type { MultipleChoiceQuestion } from "$lib/types/Exam";

  export let onEdit: (q: MultipleChoiceQuestion) => void;
  export let q: MultipleChoiceQuestion = {qtype: "MultipleChoiceQuestion", question: "Example question", answers: [], correct_id: -1};

  export let isEditing = false;

  let _newMCQ: string;
</script>

<div>
    <Badge qtype="MultipleChoiceQuestion" />
    {#if isEditing}
		<label for="question">Question:</label>
		<textarea id="question" bind:value={q.question} placeholder="Question" />

		{#each q.answers as ans, i}
			<Input
                id={`answer-${i}`}
				klazz={`${q.correct_id == i ? 'bold' : ''}`}
				bind:value={ans}
				placeholder="Answer for MCQ"
                label= {`Answer #${i+1} ${q.correct_id == i ? "[CORRECT]" : ""}`}
			/>
			<Button click={() => (q.correct_id = i)}>Make Correct answer</Button>
		{/each}

		<input bind:value={_newMCQ} placeholder="New answer" />
		<Button
			click={() => {
				q.answers = [...q.answers, _newMCQ];
                _newMCQ = ""
			}}>New answer</Button
		>

        <Button click={() => {onEdit(q); isEditing = false;}}>Save</Button>
    {:else}
        {q.question}
        {#each q.answers as a, i}
            {#if q.correct_id == i}
                <b>{a}</b>
            {:else}
                <p>{a}</p>
            {/if}
        {/each}

        <Button click={() => isEditing = true}>Edit question</Button>
    {/if}
</div>
