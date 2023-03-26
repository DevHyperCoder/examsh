<script lang="ts">
	export let placeholder: string;
	export let id: string;
	export let value: string;
	export let klazz: string = '';
	export let label: string;
</script>

<div class="flex flex-col gap-1">
	<label class="font-bold text-stone-800" for={id}>{label}</label>
	<textarea
		class={`
                    block
                    rounded-md
                    bg-gray-100
                    border-transparent
                    focus:border-gray-500 focus:bg-white focus:ring-0
        ${klazz}`}
		bind:value
		{placeholder}
	/>
</div>
