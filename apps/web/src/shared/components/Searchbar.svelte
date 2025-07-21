<script lang="ts">
	interface Props {
		label: string
		search: string
		data: any[]
	}

	let { search = $bindable(), data, label }: Props = $props()

	function handleChange(event: Event) {
		const input = event.target as HTMLInputElement
		search = input.value
	}
</script>

<section>
	<label for="search">{label}</label>
	<input
		id="search"
		type="text"
		bind:value={search}
		onchange={handleChange}
	/>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 50%;
	}

	label {
		font-weight: 500;
		margin-bottom: 0.25rem;
		font-size: 1rem;
		color: #222;
	}

	input {
		padding: 0.6em 1em;
		height: 44px;
		border-radius: 0.4em;
		border: 1px solid #e0e0e0;
		background: #f5f5f5;
		font-size: 1rem;
		transition: border 0.2s;
		box-sizing: border-box;
		width: 100%;
	}

	input:focus {
		outline: none;
		border: 1.5px solid #bdbdbd;
	}
</style>
