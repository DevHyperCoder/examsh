<script lang="ts">
	import Badge from '$lib/Badge.svelte';
	import Button from '$lib/Button.svelte';
	import TextInput from '$lib/TextInput.svelte';

	import type { RawQuestion } from '$lib/types/Exam';

	export let onEdit: (q: RawQuestion) => void;
	export let q: RawQuestion;

	export let isEditing = false;
</script>

<div class="flex flex-col gap-3">
	<Badge qtype="RawQuestion" />
	{#if isEditing}
		<p><b>Note:</b>Use pure LaTeX to write your question.</p>
		<TextInput
			label="Question:"
			id="raw-question-latex"
			bind:value={q.question}
			placeholder="Question"
		/>
		<Button
			click={() => {
				onEdit(q);
				isEditing = false;
			}}>Save</Button
		>
	{:else}
		<pre>
            <code>
    {q.question}
            </code>
        </pre>

		<Button click={() => (isEditing = true)}>Edit question</Button>
	{/if}
</div>
