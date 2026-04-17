<script lang="ts">
	import { useGoogleLoginMutation } from "$lib/auth/queries"

	const mutation = useGoogleLoginMutation()

	const onClick = () => {
		mutation.mutate(undefined, {
			onSuccess: response => {
				window.location.href = response.data ?? "/"
			},
		})
	}
</script>

<div class="flex min-h-screen w-full items-center justify-center bg-gray-50 px-4">
	<div class="w-full max-w-md">
		<!-- Logo / Branding -->
		<div class="mb-8 text-center">
			<div
				class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-black"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6 text-white"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					stroke-width="2"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M12 14l9-5-9-5-9 5 9 5z"
					/>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M12 14l6.16-3.422a12.083 12.083 0 01.665 6.479A11.952 11.952 0 0012 20.055a11.952 11.952 0 00-6.824-2.998 12.078 12.078 0 01.665-6.479L12 14z"
					/>
				</svg>
			</div>
			<h1 class="text-2xl font-semibold text-gray-900">
				Sistema de Prácticas y Tesis
			</h1>
			<p class="mt-2 text-sm text-gray-500">Universidad — Ingeniería Informática</p>
		</div>

		<!-- Card -->
		<div class="rounded-xl border border-gray-200 bg-white px-8 py-10 shadow-sm">
			<div class="mb-6 text-center">
				<h2 class="text-lg font-semibold text-gray-900">Bienvenido</h2>
				<p class="mt-1 text-sm text-gray-500">
					Inicia sesión con tu cuenta institucional de Google para continuar.
				</p>
			</div>

			<!-- Google Button -->
			<button
				onclick={onClick}
				disabled={mutation.isPending}
				class="flex w-full items-center justify-center gap-3 rounded-lg border border-gray-300 bg-white px-4 py-3 text-sm font-medium text-gray-700 shadow-sm transition-all duration-200 hover:border-gray-400 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-300 disabled:cursor-not-allowed disabled:opacity-60"
			>
				{#if mutation.isPending}
					<!-- Spinner -->
					<svg
						class="h-5 w-5 animate-spin text-gray-500"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8H4z"
						></path>
					</svg>
					<span>Redirigiendo...</span>
				{:else}
					<!-- Google G Logo -->
					<svg
						class="h-5 w-5"
						viewBox="0 0 24 24"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
							fill="#4285F4"
						/>
						<path
							d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
							fill="#34A853"
						/>
						<path
							d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l3.66-2.84z"
							fill="#FBBC05"
						/>
						<path
							d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
							fill="#EA4335"
						/>
					</svg>
					<span>Continuar con Google</span>
				{/if}
			</button>

			{#if mutation.isError}
				<div
					class="mt-4 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
				>
					Ocurrió un error al iniciar sesión. Por favor, inténtalo de nuevo.
				</div>
			{/if}

			<p class="mt-6 text-center text-xs text-gray-400">
				Solo se permite el acceso con cuentas institucionales autorizadas.
			</p>
		</div>

		<!-- Footer -->
		<p class="mt-6 text-center text-xs text-gray-400">
			© {new Date().getFullYear()} Universidad — Todos los derechos reservados
		</p>
	</div>
</div>
