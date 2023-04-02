<script lang="ts">
	import type { Exam, Question } from '$lib/types/Exam';

	import CreateQuestion from '$lib/CreateQuestion.svelte';
	import Raw from './Raw.svelte';
	import Error from '$lib/Error.svelte';
	import Button from '$lib/Button.svelte';
	import { invoke } from '@tauri-apps/api';
	import Mcq from './MCQ.svelte';
	import PredictOutput from './PredictOutput.svelte';

	let creatingQuestion = false;

	async function editQuestion(q: Question, idx: number) {
		await invoke('edit_question', {
			examIdent,
			question: q
		});

		let a = exam.questions;
		a[idx] = q;
		exam.questions = a;
	}

	export let examIdent: string;
	export let exam: Exam;
</script>

<h1 class="text-2xl font-bold text-stone-800 mb-3 text-center">Questions</h1>
{#each exam.questions as q, i}
	<div class="border-double border-left border-l-4 border-sky-500 pl-4">
		{#if q.qtype == 'MultipleChoiceQuestion'}
			<Mcq onEdit={async (newq) => editQuestion(newq, i)} {q} />
		{:else if q.qtype == 'RawQuestion'}
			<Raw onEdit={async (newq) => editQuestion(newq, i)} {q} />
		{:else if q.qtype == 'PredictOutputQuestion'}
			<PredictOutput onEdit={async (newq) => editQuestion(newq, i)} {q} />
		{/if}
	</div>
{:else}
	<Error err={'No questions present'} />
{/each}

{#if creatingQuestion}
	<CreateQuestion
		newQuestionNumber={exam.questions.length + 1}
		onQuestionChanged={(qs) => {
			exam.questions = qs;
			creatingQuestion = false;
		}}
		{examIdent}
	/>
	<Button klazz="bg-red-500" click={() => (creatingQuestion = false)}>Cancel</Button>
{:else}
	<Button click={() => (creatingQuestion = true)}>Create new question</Button>
{/if}
