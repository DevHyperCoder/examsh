<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import Button from './Button.svelte';
	import Input from './Input.svelte';
    import Mcq from './question/MCQ.svelte';
import PredictOutput from './question/PredictOutput.svelte';
    import Raw from './question/Raw.svelte';
	import SelectInput from './SelectInput.svelte';
	import TextInput from './TextInput.svelte';
    import type { MultipleChoiceQuestion, PredictOutputQuestion, Question, QuestionType, RawQuestion } from './types/Exam';

	export let examIdent: string;
	export let onQuestionChanged: (newQ: any[]) => void;

	const qtypes = [
		{ name: 'Predict Output', value: 'PredictOutputQuestion' },
		{ name: 'MCQ', value: 'MultipleChoiceQuestion' },
		{ name: 'Write code', value: 'WriteCodeQuestion' },
		{ name: 'Raw', value: 'RawQuestion' }
	];


	let qtype: QuestionType = "PredictOutputQuestion";

	let rawQuestion: RawQuestion = {qtype: "RawQuestion", question: ""};

	let multipleChoiceQuestion: MultipleChoiceQuestion = {
        qtype: "MultipleChoiceQuestion", 
		question: '',
		answers: [],
		correct_id: 0
	};

	let predictOutputQuestion: PredictOutputQuestion = {
        qtype: "PredictOutputQuestion",
		question: '',
		pre_run: '',
		run: '',
		post_run: '',
		_code: [["",""]]
	};

    const qtypesToValues : Record<QuestionType, Question> = {
        "RawQuestion": rawQuestion,
        "MultipleChoiceQuestion": multipleChoiceQuestion,
        "PredictOutputQuestion": predictOutputQuestion
    };

	async function addQuestion() {
		try {
			onQuestionChanged(
				await invoke('add_question', {
					examIdent,
					question: qtypesToValues[qtype]
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
        <!-- Creating raw q -->
        <Raw isEditing onEdit={(r) => {qtypesToValues["RawQuestion"] = r; addQuestion();}} />
	{:else if qtype == 'MultipleChoiceQuestion'}
		<Mcq isEditing onEdit={(m) => {qtypesToValues["MultipleChoiceQuestion"] = m; addQuestion();}} />
	{:else if qtype == 'PredictOutputQuestion'}
		<PredictOutput isEditing onEdit={(m) => {qtypesToValues["PredictOutputQuestion"] = m; addQuestion();}} />
	{:else}
		<p>unimplemented</p>
	{/if}

    {#if false}
	<Button klazz="w-max ml-auto" disabled={qtypes[2].value == qtype} click={addQuestion}>
		Add Question</Button
	>
    {/if}
</form>
