<script lang="ts">
	import { Portal } from "@ark-ui/svelte/portal"
	import { ChevronDownIcon } from "lucide-svelte"
	import { Select, createListCollection } from "@ark-ui/svelte/select"

	interface Props {
		roles: string[]
	}

	let { roles = $bindable() }: Props = $props()

	interface Item {
		label: string
		value: string
		disabled?: boolean
	}

	const collection = createListCollection<Item>({
		items: [
			{ label: "Estudiante", value: "student" },
			{ label: "Profesor", value: "teacher" },
			{ label: "Administrador", value: "administrator" },
			{ label: "Coordinador", value: "coordinator" },
			{ label: "Secretario (a)", value: "secretary" },
		],
	})
</script>

<Select.Root {collection} bind:value={roles} multiple>
	<Select.Label>Filtrar por roles de usuario</Select.Label>
	<Select.Control>
		<Select.Trigger>
			<Select.ValueText placeholder="Selecciona uno o más roles" />
			<Select.Indicator>
				<ChevronDownIcon />
			</Select.Indicator>
		</Select.Trigger>
	</Select.Control>
	<Portal>
		<Select.Positioner>
			<Select.Content>
				<Select.ItemGroup>
					{#each collection.items as item (item.value)}
						<Select.Item {item}>
							<Select.ItemText>{item.label}</Select.ItemText>
							<Select.ItemIndicator>✓</Select.ItemIndicator>
						</Select.Item>
					{/each}
				</Select.ItemGroup>
			</Select.Content>
		</Select.Positioner>
	</Portal>
	<Select.HiddenSelect />
</Select.Root>

<style>
	:global([data-part="root"]) {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		width: 30%;
		height: 100%;
	}

	:global([data-part="label"]) {
		font-weight: 500;
		margin-bottom: 0.25rem;
		font-size: 1rem;
		color: #222;
	}

	:global([data-part="control"]) {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
	}

	:global([data-part="trigger"]) {
		padding: 0.6em 1em;
		height: 44px;
		border-radius: 0.4em;
		border: 1px solid #e0e0e0;
		background: #f5f5f5;
		font-size: 1rem;
		transition: border 0.2s;
		box-sizing: border-box;
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	:global([data-part="positioner"]) {
		background: #f5f5f5;
		overflow-y: hidden;
		overflow-x: hidden;
		border-radius: 0.4rem;
		display: flex;
		flex-direction: column;
	}

	:global([data-part="content"]) {
		background: #f5f5f5;
		border: 1px solid #ddd;
		border-radius: 0.4rem;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
		padding: 0.5rem;
	}

	:global([data-part="item-group"]) {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	:global([data-part="item"]) {
		padding: 0.5rem 0.5rem;
		border-radius: 0.4rem;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
</style>
