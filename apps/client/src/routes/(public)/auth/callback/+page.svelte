<script lang="ts">
	import { goto } from "$app/navigation"
	import { onMount } from "svelte"
	import { auth } from "$lib/auth/store.svelte"

	let error = $state<string | null>(null)
	let isProcessing = $state(true)

	onMount(() => {
		processCallback()
	})

	async function processCallback() {
		try {
			// 1. Procesar tokens desde cookies y guardar en localStorage
			const success = auth.processOAuthCallback()

			if (!success) {
				error = "No se encontraron tokens de autenticación válidos"
				setTimeout(() => goto("/auth/login"), 2000)
				return
			}

			// 2. Obtener información del usuario
			const tokens = auth.getTokens()
			if (!tokens) {
				error = "Error al obtener tokens"
				setTimeout(() => goto("/auth/login"), 2000)
				return
			}

			// 3. Decodificar para obtener el rol y redirigir
			const claims = auth.decodeToken(tokens.accessToken)
			if (!claims) {
				error = "Token inválido"
				auth.reset()
				setTimeout(() => goto("/auth/login"), 2000)
				return
			}

			// 4. Redirigir al dashboard según el rol
			const routes = {
				administrator: "/admin",
				teacher: "/teacher",
				secretary: "/secretary",
				student: "/student",
			} as const

			const destination = routes[claims.role] ?? "/"
			goto(destination, { replaceState: true })
		} catch (err) {
			console.error("Error processing callback:", err)
			error = "Error al procesar la autenticación"
			auth.reset()
			setTimeout(() => goto("/auth/login"), 2000)
		} finally {
			isProcessing = false
		}
	}
</script>

{#if error}
	<div class="flex h-screen items-center justify-center bg-gray-50">
		<div class="rounded-lg bg-white p-8 text-center shadow-lg">
			<div
				class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-red-100"
			>
				<svg
					class="h-6 w-6 text-red-600"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/>
				</svg>
			</div>
			<h2 class="mb-2 text-xl font-semibold text-gray-900">
				Error de autenticación
			</h2>
			<p class="text-gray-600">{error}</p>
			<p class="mt-2 text-sm text-gray-500">Redirigiendo al login...</p>
		</div>
	</div>
{:else if isProcessing}
	<div class="flex h-screen items-center justify-center bg-gray-50">
		<div class="text-center">
			<div
				class="mx-auto mb-4 h-12 w-12 animate-spin rounded-full border-4 border-blue-500 border-t-transparent"
			></div>
			<h2 class="mb-2 text-xl font-semibold text-gray-900">Autenticando</h2>
			<p class="text-gray-600">Procesando tu información...</p>
		</div>
	</div>
{/if}
