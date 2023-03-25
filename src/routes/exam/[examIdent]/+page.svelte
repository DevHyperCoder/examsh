<script lang="ts">
	import { page } from '$app/stores';
	import CreateQuestion from '$lib/CreateQuestion.svelte';
	import { invoke, dialog } from '@tauri-apps/api';

	export let { examIdent } = $page.params;
	const e = async () => {
		if (examIdent) {
			try {
				exam = await invoke('load_exam_with_ident', { examIdent: examIdent });
			} catch (e) {
				err = e as string;
			}
		}
	};

	let err = '';
	let exam: any;
	e();

	async function render() {
		try {
			await invoke('render_exam', { examIdent });
		} catch (e) {
			await dialog.message(e as string, { title: 'Error!', type: 'error' });
		}
	}
</script>

{#if exam}
	<p><b>Course name: </b>{exam.exam_schema.course_name}</p>
	<p><b>Test name: </b>{exam.exam_schema.test_name}</p>
	<p><b>Instructions for candidates: </b>{exam.exam_schema.instructions}</p>
	<p><b>Instructions for evaluation: </b>{exam.exam_schema.marking_instructions}</p>

	<h2>Questions</h2>
	{#each exam.questions as q}
		<p>[<span>{q.qtype}</span>] {q.question}</p>

		{#if q.qtype == 'MultipleChoiceQuestion'}
			{#each q.answers as a, i}
				{#if q.correct_id == i}
					<b>{a}</b>
				{:else}
					<p>{a}</p>
				{/if}
			{/each}
		{:else}
			<pre>{JSON.stringify(q, null, 4)}</pre>
		{/if}
	{/each}

	<CreateQuestion onQuestionChanged={(qs) => (exam.questions = qs)} {examIdent} />

	<button on:click={render}>Make Question PDF and answer PDF</button>
{/if}

<p>{err}</p>
<a href="../">go back</a>
