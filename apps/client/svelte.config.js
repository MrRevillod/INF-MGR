import adapter from "@sveltejs/adapter-static"
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte"

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: "index.html",
		}),
		alias: {
			$lib: "src/lib",
			$api: "src/lib/shared/api",
			$components: "src/lib/components",
			$stores: "src/lib/stores",
			$shared: "src/lib/shared",
			$users: "src/lib/users",
			$courses: "src/lib/courses",
		},
	},
}

export default config
