<script lang="ts">
	export let klazz: string = '';
	export let click: (() => void) | (() => Promise<void>);
	export let disabled: boolean = false;
</script>

<button
	{disabled}
	on:click={click}
	class={`bg-blue-500 text-white font-semibold px-2 py-1 rounded-md ${klazz}`}
>
	<slot />
</button>
