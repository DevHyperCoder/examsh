<script lang="ts">
	import { goto } from '$app/navigation';
	import Error from '$lib/Error.svelte';
	import { dialog, invoke } from '@tauri-apps/api';
	import CreateExam from '../lib/CreateExam.svelte';

	async function loadExam() {
		const directory = await dialog.open({
			directory: true,
			title: 'Choose the folder with your exam',
			multiple: false,
			recursive: false
		});

		if (!directory) {
			err = 'No directory chosen.';
			return;
		}

		try {
			const val: [any, string] = await invoke('load_exam', {
				directory
			});
			goto(`/exam/${val[1]}`);
			err = '';
		} catch (e) {
			err = e as any;
		}
	}

	let err = '';

	let showingCreate = false;
</script>

<h1>Welcome to examsh</h1>

<button
	class="px-6 py-12 bg-blue-100 hover:text-white hover:bg-blue-400 font-bold text-xl"
	on:click={async () => {
		showingCreate = false;
		await loadExam();
	}}>Load existing exam</button
>
<button
	class="px-6 py-12 bg-blue-100 hover:text-white hover:bg-blue-400 font-bold text-xl"
	on:click={() => (showingCreate = true)}>Create new exam</button
>

<Error {err} />

{#if showingCreate}
	<CreateExam />
{/if}
