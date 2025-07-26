<script lang="ts">
	import Navbar from "$components/Navbar.svelte"
	import "../app.css"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	let { children } = $props()

	const queryClient = new QueryClient()
</script>

<QueryClientProvider client={queryClient}>
	<main class="bg-background max-h-screen min-h-screen">
		<Navbar />

		<section
			class="bg-background ml-16 min-h-screen transition-all duration-300 lg:ml-64"
		>
			<div class="p-4 lg:p-8">
				{@render children()}
			</div>
		</section>
	</main>
</QueryClientProvider>
