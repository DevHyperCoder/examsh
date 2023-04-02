<script lang="ts">
	import { page } from '$app/stores';
	import Button from '$lib/Button.svelte';
	import ExamDetail from '$lib/exam/ExamDetail.svelte';
	import QuestionsList from '$lib/question/QuestionsList.svelte';
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

<section class="w-1/2 mx-auto flex flex-col gap-5">
	{#if exam}
		<ExamDetail {examIdent} examSchema={exam.exam_schema} />

		<QuestionsList {exam} {examIdent} />

		<Button click={render}>Make Question PDF and answer PDF</Button>
	{/if}

	<p>{err}</p>
	<a href="../">go back</a>
</section>
