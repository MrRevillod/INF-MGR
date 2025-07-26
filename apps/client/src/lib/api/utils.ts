import type { AxiosResponse } from "axios"
import type { ApiResponse } from "$lib/types"

export const appendQueryParams = (
	url: string,
	params: Record<string, string | undefined>
) => {
	const query = new URLSearchParams()

	for (const [key, value] of Object.entries(params)) {
		if (value !== undefined) {
			query.append(key, value)
		}
	}

	return `${url}${query.toString() ? `?${query.toString()}` : ""}`
}

export const UknownError = {
	data: null,
	status: 500,
	timestamp: new Date().toISOString(),
	message: "Error desconocido, por favor intente más tarde.",
}

export const tryHttp = async <T>({
	fn,
}: {
	fn: AxiosResponse<T>
}): Promise<ApiResponse<T>> => {
	try {
		return (await fn.data) as ApiResponse<T>
	} catch (error: any) {
		console.error("Error in tryHttp:", error)
		return error?.response?.data ?? UknownError
	}
}
