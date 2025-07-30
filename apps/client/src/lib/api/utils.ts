import type { ApiResponse } from "$lib/types"
import type { AxiosResponse } from "axios"

export const UknownError = {
	data: null,
	status: 500,
	timestamp: new Date().toISOString(),
	message: "Error desconocido, por favor intente más tarde.",
}

interface TryHttpParams<T> {
	fn: () => Promise<AxiosResponse<T>>
}

export const tryHttp = async <T>(
	props: TryHttpParams<T>
): Promise<ApiResponse<T>> => {
	try {
		return (await props.fn()).data as ApiResponse<T>
	} catch (error: any) {
		console.error("Error in tryHttp:", error)
		return error?.response?.data ?? UknownError
	}
}
