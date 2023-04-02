<script lang="ts">
	import { goto } from '$app/navigation';

	import { dialog, invoke } from '@tauri-apps/api';
	import Button from '$lib/Button.svelte';
	import Error from '$lib/Error.svelte';
	import Input from '$lib/Input.svelte';
	import TextInput from '$lib/TextInput.svelte';
	import type { ExamSchema } from '$lib/types/ExamSchema';

	async function createNewExam() {
		const directory = await dialog.open({
			directory: true,
			title: 'Choose a folder to store your new exam',
			multiple: false,
			recursive: false
		});

		if (!directory) {
			err = 'No directory chosen.';
			return;
		}
		try {
			const val: [any, string] = await invoke('create_new_exam', {
				directory,
				examSchema: examSchema
			});
			goto(`/exam/${val[1]}`);
			err = '';
		} catch (e) {
			err = e as any;
		}
	}

	async function editExam() {
		try {
			const val: [any, string] = await invoke('edit_exam_schema', {
				examIdent: editingExamIdent,
				examSchema
			});

			console.log('calling onEdit');
			onEdit!(examSchema);

			if (editingExamIdent != val[1]) {
				goto(`/exam/${val[1]}`);
			}
			err = '';
		} catch (e) {
			err = e as any;
		}
	}

	export let examSchema: ExamSchema = {
		course_name: '',
		test_name: 'Example test',
		instructions: 'Read all questions carefully',
		marking_instructions: 'Mark properly',
		date_fmt: null
	};

	export let editingExamIdent: string | null = null;
	export let onEdit: ((newEx: ExamSchema) => void) | null = null;

	let err = '';
</script>

<h1 class="text-center text-2xl font-bold text-stone-800 mb-3">
	{editingExamIdent ? 'Edit' : 'Create new'} exam
</h1>

<form class="flex flex-col gap-3">
	<Input
		label="Course name:"
		id="course-name"
		placeholder="Enter new course name: "
		bind:value={examSchema.course_name}
	/>

	<Input
		label="Test name:"
		id="test-name"
		placeholder="Enter new test name: "
		bind:value={examSchema.test_name}
	/>

	<TextInput
		label="Candidate instructions: "
		id="c-instructions"
		placeholder="Enter instructions for the candidates"
		bind:value={examSchema.instructions}
	/>

	<TextInput
		label="Marking instructions: "
		id="m-instructions"
		placeholder="Enter instructions for the evaluators"
		bind:value={examSchema.marking_instructions}
	/>

	<Error {err} />

	<Button klazz="w-max ml-auto text-lg" click={editingExamIdent ? editExam : createNewExam}
		>{`${editingExamIdent ? 'Edit' : 'Create new'} exam`}</Button
	>
</form>
