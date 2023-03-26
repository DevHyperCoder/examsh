<script lang="ts">
	import { goto } from '$app/navigation';

	import { dialog, invoke } from '@tauri-apps/api';
	import Button from './Button.svelte';
	import Error from './Error.svelte';
	import Input from './Input.svelte';
	import TextInput from './TextInput.svelte';

	async function createNewExam() {
		const directory = await dialog.open({
			directory: true,
			title: 'Choose a folder to store your new exam',
			multiple: false,
			recursive: false
		});

		if (!directory) {
			str = 'No directory chosen.';
			return;
		}
		try {
			const val: [any, string] = await invoke('create_new_exam', {
				directory,
				examSchema: exam_schema
			});
			goto(`/exam/${val[1]}`);
			str = '';
		} catch (e) {
			str = e as any;
		}
	}

	let exam_schema: {
		course_name: string;
		test_name: string;
		instructions: string;
		marking_instructions: string;
	} = {
		course_name: '',
		test_name: 'Example test',
		instructions: 'Read all questions carefully',
		marking_instructions: 'Mark properly'
	};
	let str = '';
</script>

<h1 class="text-center text-2xl font-bold text-stone-800 mb-3">Create new exam</h1>

<form class="w-1/2 mx-auto flex flex-col gap-3">
	<Input
		label="Course name:"
		id="course-name"
		placeholder="Enter new course name: "
		bind:value={exam_schema.course_name}
	/>

	<Input
		label="Test name:"
		id="test-name"
		placeholder="Enter new test name: "
		bind:value={exam_schema.test_name}
	/>

	<TextInput
		label="Candidate instructions: "
		id="c-instructions"
		placeholder="Enter instructions for the candidates"
		bind:value={exam_schema.instructions}
	/>

	<TextInput
		label="Marking instructions: "
		id="m-instructions"
		placeholder="Enter instructions for the evaluators"
		bind:value={exam_schema.marking_instructions}
	/>

	<Error err={str} />

	<Button klazz="w-max ml-auto text-lg" click={createNewExam}>Create new exam</Button>
</form>
