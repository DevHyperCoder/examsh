<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import Button from './Button.svelte';
	import Input from './Input.svelte';
	import SelectInput from './SelectInput.svelte';
	import TextInput from './TextInput.svelte';

	export let examIdent: string;
	export let onQuestionChanged: (newQ: any[]) => void;

	const qtypes = [
		{ name: 'Predict Output', value: 'PredictOutputQuestion' },
		{ name: 'MCQ', value: 'MultipleChoiceQuestion' },
		{ name: 'Write code', value: 'WriteCodeQuestion' },
		{ name: 'Raw', value: 'RawQuestion' }
	];

	let qtype: string;

	let rawQuestion: { latex: string } = { latex: '' };

	let multipleChoiceQuestion: { question: string; answers: string[]; correct_id: number } = {
		question: '',
		answers: [],
		correct_id: 0
	};
	let _newMCQ: string;

	let predictOutputQuestion: {
		question: string;
		pre_run: string;
		run: string;
		post_run: string;
		_code: string[][];
	} = {
		question: '',
		pre_run: '',
		run: '',
		post_run: '',
		_code: []
	};

	async function addQuestion() {
		let question = null;
		if (qtype == 'RawQuestion') {
			question = { qtype, ...rawQuestion };
		} else if (qtype == 'MultipleChoiceQuestion') {
			question = { qtype, ...multipleChoiceQuestion };
		} else {
			question = { qtype, ...predictOutputQuestion };
			console.error('panic!');
		}
		try {
			onQuestionChanged(
				await invoke('add_question', {
					examIdent,
					question
				})
			);
		} catch (e) {
			console.error(e);
		}
	}
</script>

<form class="w-1/2 mx-auto flex flex-col gap-3">
	<SelectInput id="question-type" items={qtypes} bind:value={qtype} label="Question type:" />

	{#if qtype == 'RawQuestion'}
		<p><b>Note:</b>Use pure LaTeX to write your question.</p>
		<TextInput
			label="Question:"
			id="raw-question-latex"
			bind:value={rawQuestion.latex}
			placeholder="Question"
		/>
	{:else if qtype == 'MultipleChoiceQuestion'}
		<label for="question">Question:</label>
		<textarea id="question" bind:value={multipleChoiceQuestion.question} placeholder="Question" />

		{#each multipleChoiceQuestion.answers as ans, i}
			<input
				class={`${multipleChoiceQuestion.correct_id == i ? 'bold' : ''}`}
				bind:value={ans}
				placeholder="Answer for MCQ"
			/>
			<button on:click={() => (multipleChoiceQuestion.correct_id = i)}>Make Correct answer</button>
		{/each}

		<input bind:value={_newMCQ} placeholder="New answer" />
		<button
			on:click={() => {
				multipleChoiceQuestion.answers = [...multipleChoiceQuestion.answers, _newMCQ];
			}}>New answer</button
		>
	{:else if qtype == 'PredictOutputQuestion'}
		<TextInput
			id="predict-output-question"
			placeholder="Enter question"
			bind:value={predictOutputQuestion.question}
			label="Question:"
		/>

		<Input
			placeholder="Enter pre command"
			label="Command to run BEFORE execution (compilation etc)"
			id="pre-run"
			bind:value={predictOutputQuestion.pre_run}
		/>
		<Input
			placeholder="Enter run command"
			label="Command to run FOR execution (actuaion running of the program)"
			id="run"
			bind:value={predictOutputQuestion.run}
		/>
		<Input
			placeholder="Enter post command"
			label="Command to run AFTER execution (cleanup etc)"
			id="post-run"
			bind:value={predictOutputQuestion.post_run}
		/>

		<h2 class="text-lg font-semibold text-stone-800">Code files:</h2>

		{#each predictOutputQuestion._code as code_file}
			<Input label="File Name: " id="fname" bind:value={code_file[0]} placeholder="Filename" />
			<TextInput label="Code: " id="code" bind:value={code_file[1]} placeholder="Code" />
		{/each}

		<Button
			klazz="w-max mr-auto"
			click={() => (predictOutputQuestion._code = [...predictOutputQuestion._code, ['', '']])}
			>Create new file</Button
		>
	{:else}
		<p>unimplemented</p>
	{/if}

	<Button klazz="w-max ml-auto" disabled={qtypes[2].value == qtype} click={addQuestion}>
		Add Question</Button
	>
</form>
