<script lang="ts">
	import "../app.css"

	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	import Navbar from "$lib/components/ui/Navbar.svelte"

	let { children } = $props()

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<main class="bg-background max-h-screen min-h-screen">
		<Navbar />

		<section
			class="bg-background ml-16 min-h-screen transition-all duration-300 lg:ml-64"
		>
			<div class="px-4 py-8 lg:p-8">
				{@render children()}
			</div>
		</section>
	</main>
</QueryClientProvider>
