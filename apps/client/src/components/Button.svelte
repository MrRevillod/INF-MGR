<script lang="ts">
	interface ButtonProps {
		onclick: () => void
		variant: "primary" | "secondary"
		disabled?: boolean
		text: string
	}

	const variantClasses = {
		primary: `bg-accent hover:bg-accent-hover focus:ring-focus-ring inline-flex items-center 
            gap-2 rounded-lg px-4 py-2.5 text-sm font-medium text-white transition-all duration-200
            focus:outline-none focus:ring-2
        `,
		secondary: `text-text-primary bg-surface border-border hover:bg-hover-bg focus:ring-focus-ring 
			inline-flex items-center gap-2 rounded-lg border px-4 py-2.5 text-sm font-medium 
			transition-all duration-200 focus:outline-none focus:ring-2 disabled:cursor-not-allowed 
			disabled:opacity-50
		`,
	}

	let {
		onclick,
		text = "Button",
		variant = "primary",
		disabled = false,
	}: ButtonProps = $props()
</script>

<button {onclick} class={variantClasses[variant]} {disabled}>
	{text}
</button>
