<script lang="ts">
	import Badge from '$lib/Badge.svelte';
	import Button from '$lib/Button.svelte';
	import Input from '$lib/Input.svelte';

	import type { MultipleChoiceQuestion } from '$lib/types/Exam';

	export let onEdit: (q: MultipleChoiceQuestion) => void;
	export let q: MultipleChoiceQuestion;

	export let isEditing = false;

	let _newMCQ: string;
	let makingNewAnswer = false;
</script>

<div class="flex flex-col gap-3">
	<Badge qtype="MultipleChoiceQuestion" />
	{#if isEditing}
		<div class="flex flex-col gap-2">
			<label for="question">Question:</label>
			<textarea id="question" bind:value={q.question} placeholder="Question" />
		</div>

		<div>
			{#each q.answers as ans, i}
				<div class="flex flex-col gap-2">
					<Input
						id={`answer-${i}`}
						klazz={`${q.correct_id == i ? 'bold' : ''}`}
						bind:value={ans}
						placeholder="Answer for MCQ"
						label={`Answer #${i + 1} ${q.correct_id == i ? '[CORRECT]' : ''}`}
					/>
					<Button click={() => (q.correct_id = i)}>Make Correct answer</Button>
				</div>
			{/each}
		</div>

		{#if makingNewAnswer}
			<div>
				<Input label="New answer" id="new-answer" bind:value={_newMCQ} placeholder="New answer" />
				<Button
					click={() => {
						q.answers = [...q.answers, _newMCQ];
						_newMCQ = '';
					}}>New answer</Button
				>
			</div>
		{:else}
			<Button click={() => (makingNewAnswer = true)}>Add new option</Button>
		{/if}

		<Button
			click={() => {
				onEdit(q);
				isEditing = false;
			}}>Save</Button
		>
	{:else}
		{q.question}
		{#each q.answers as a, i}
			{#if q.correct_id == i}
				<b>{a}</b>
			{:else}
				<p>{a}</p>
			{/if}
		{/each}

		<Button click={() => (isEditing = true)}>Edit question</Button>
	{/if}
</div>
