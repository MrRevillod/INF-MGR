<script lang="ts">
	import { onMount } from "svelte"
	import { goto } from "$app/navigation"
	import { auth } from "$lib/auth/store.svelte"

	onMount(() => {
		const user = auth.user

		if (!user) {
			goto("/auth/login")
			return
		}

		// Redirigir según el rol del usuario
		switch (user.role) {
			case "administrator":
				goto("/admin")
				break
			case "secretary":
				goto("/secretary")
				break
			case "teacher":
				goto("/teacher")
				break
			case "student":
				goto("/student")
				break
			default:
				goto("/auth/login")
		}
	})
</script>

<div class="flex min-h-screen items-center justify-center">
	<div class="text-gray-500">Redirigiendo...</div>
</div>
