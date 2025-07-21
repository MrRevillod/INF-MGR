import { apiClient } from "./axios.svelte"

export type HttpResponse<T> = {
	status: number
	data: T | null
	success: boolean
	message: string
	timestamp: string
}

export type RequestOptions = {
	url: string
	method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH"
	headers?: Record<string, string>
	body?: any
	query?: Record<string, any>
	enabled?: boolean
}

export class Request<T = unknown> {
	public isLoading = $state(false)
	public response = $state<HttpResponse<T> | null>(null)
	public isError = $state(false)
	public data = $derived(this.response?.data ?? null)

	public options = $state<RequestOptions | null>(null)

	#refetchTrigger = $state(0)

	constructor(initialOptions?: RequestOptions) {
		this.options = initialOptions ?? null

		$effect(() => {
			const opts = this.options
			const _ = this.#refetchTrigger

			const q = opts?.query
			if (q) Object.values(q)

			if (!opts || opts.enabled === false) return

			this.fetch()
		})
	}

	public refetch() {
		this.#refetchTrigger++
	}

	private formatUrl(url: string, query?: Record<string, any>): string {
		if (!query || Object.keys(query).length === 0) return url

		const queryParams = Object.entries(query)
			.filter(([_, value]) => value != null)
			.map(([key, value]) => {
				if (Array.isArray(value)) value = value.join(",")
				else if (typeof value === "object")
					value = JSON.stringify(value)
				return `${encodeURIComponent(key)}=${encodeURIComponent(value)}`
			})
			.join("&")

		return `${url}?${queryParams}`
	}

	private async fetch() {
		const opts = this.options
		if (!opts) return

		this.isLoading = true

		const formattedUrl = this.formatUrl(opts.url, opts.query)

		try {
			const { data } = await apiClient.request<HttpResponse<T>>({
				url: formattedUrl,
				method: opts.method ?? "GET",
				data: opts.body ?? null,
				headers: opts.headers ?? {},
			})

			this.isError = false
			this.response = data
		} catch (error: any) {
			this.isError = true
			this.response = error.response?.data ?? null
		}

		this.isLoading = false
	}
}
