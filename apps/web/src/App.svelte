<script lang="ts">
	import { Router, type RouteConfig } from "@mateothegreat/svelte5-router"

	import UsersPage from "./features/users/components/Page.svelte"

	const routes: RouteConfig[] = [
		{
			path: "/usuarios",
			component: UsersPage,
		},
	]
</script>

<Router {routes} />
