<script lang="ts">
	import CreateEditExam from './CreateEditExam.svelte';
	import type { ExamSchema } from '$lib/types/ExamSchema';
	import Button from '$lib/Button.svelte';

	export let examSchema: ExamSchema;
	export let examIdent: string;

	let isEditing = false;

	function updateExamSchema(ex: ExamSchema) {
		console.log('called');
		examSchema = ex;
		isEditing = false;
	}
</script>

<div>
	{#if isEditing}
		<CreateEditExam {examSchema} editingExamIdent={examIdent} onEdit={updateExamSchema} />
	{:else}
		<div class="text-lg mb-5">
			<h1 class="text-2xl font-bold text-stone-800 mb-3 text-center">Exam details</h1>
			<p><b>Course name: </b>{examSchema.course_name}</p>
			<p><b>Test name: </b>{examSchema.test_name}</p>
			<p><b>Instructions for candidates: </b>{examSchema.instructions}</p>
			<p><b>Instructions for evaluation: </b>{examSchema.marking_instructions}</p>

			<Button click={() => (isEditing = true)}>Edit details</Button>
		</div>
	{/if}
</div>
