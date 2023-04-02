<script lang="ts">
	import Badge from '$lib/Badge.svelte';
	import Button from '$lib/Button.svelte';
	import Input from '$lib/Input.svelte';
	import TextInput from '$lib/TextInput.svelte';

	import type { PredictOutputQuestion } from '$lib/types/Exam';

	export let onEdit: (q: PredictOutputQuestion) => void;
	export let q: PredictOutputQuestion;

	export let isEditing = false;
</script>

<Badge qtype="PredictOutputQuestion" />
{#if isEditing}
	<TextInput
		id="predict-output-question"
		placeholder="Enter question"
		bind:value={q.question}
		label="Question:"
	/>

	<Input
		placeholder="Enter pre command"
		label="Command to run BEFORE execution (compilation etc)"
		id="pre-run"
		bind:value={q.pre_run}
	/>

	<Input
		placeholder="Enter run command"
		label="Command to run FOR execution (actuaion running of the program)"
		id="run"
		bind:value={q.run}
	/>

	<Input
		placeholder="Enter post command"
		label="Command to run AFTER execution (cleanup etc)"
		id="post-run"
		bind:value={q.post_run}
	/>

	<h2 class="text-lg font-semibold text-stone-800">Code files:</h2>

	{#each q._code as code_file}
		<Input label="File Name: " id="fname" bind:value={code_file[0]} placeholder="Filename" />
		<TextInput label="Code: " id="code" bind:value={code_file[1]} placeholder="Code" />
	{/each}

	<Button klazz="w-max mr-auto" click={() => (q._code = [...q._code, ['', '']])}
		>Create new file</Button
	>

	<Button
		click={() => {
			onEdit(q);
			isEditing = false;
		}}>Save</Button
	>
{:else}
	{q.question}

	<h2>Code files</h2>
	{#each q._code as code_file}
		<p>{code_file[0]}</p>
		<pre><code>{code_file[1]}</code></pre>
	{/each}

	<h2>Commands</h2>
	<pre><code>
# Pre run
{q.pre_run}

# Run
{q.run}

# Post run
{q.post_run}
    </code></pre>

	<Button click={() => (isEditing = true)}>Edit question</Button>
{/if}
