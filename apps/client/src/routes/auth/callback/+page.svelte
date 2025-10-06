<script lang="ts">
	import { goto } from "$app/navigation"
	import { jwtDecode } from "jwt-decode"

	interface TokenClaims {
		user_id: string
		session_id: string
		role: "administrator" | "student" | "teacher" | "secretary"
		exp: number
	}

	const roleRoutes = {
		administrator: "/admin",
		teacher: "/teacher",
		secretary: "/secretary",
		student: "/student",
	} as const

	const getCookie = (name: string): string | undefined => {
		const value = `; ${document.cookie}`
		const parts = value.split(`; ${name}=`)
		if (parts.length === 2) return parts.pop()?.split(";").shift()
	}

	$effect(() => {
		const [access, refresh] = [getCookie("ACCESS"), getCookie("REFRESH")]

		if (!access || !refresh) {
			goto("/auth/login")
			return
		}

		localStorage.setItem("ACCESS", access)
		localStorage.setItem("REFRESH", refresh)

		try {
			const claims = jwtDecode<TokenClaims>(access)
			const route = roleRoutes[claims.role] ?? "/"
			goto(route)
		} catch (error) {
			console.error("Error decoding token:", error)
			goto("/auth/login")
		}
	})
</script>

<div class="flex h-screen items-center justify-center">
	<div class="text-center">
		<h1 class="text-2xl font-semibold text-gray-900">Procesando autenticación...</h1>
		<p class="mt-2 text-gray-600">Redirigiendo...</p>
		<div class="mt-4">
			<div
				class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
			></div>
		</div>
	</div>
</div>
