import { includeIgnoreFile } from "@eslint/compat"
import { fileURLToPath } from "node:url"

import js from "@eslint/js"
import svelte from "eslint-plugin-svelte"
import globals from "globals"
import prettier from "eslint-config-prettier"

import ts from "typescript-eslint"
import svelteConfig from "./svelte.config.js"

const gitignorePath = fileURLToPath(new URL("../../.gitignore", import.meta.url))

export default ts.config(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs.recommended,
	prettier,
	...svelte.configs.prettier,
	{
		languageOptions: {
			globals: { ...globals.browser, ...globals.node },
		},
		rules: {
			"no-undef": "off",
		},
	},
	{
		files: ["**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: [".svelte"],
				parser: ts.parser,
				svelteConfig,
			},
		},
	}
)
