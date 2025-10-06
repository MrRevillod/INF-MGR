import type { ApiResponse } from "$api/types"
import axios, { type AxiosResponse } from "axios"

export const UknownError = {
	data: null,
	status: 500,
	timestamp: new Date().toISOString(),
	message: "Error desconocido, por favor intente más tarde.",
}

interface TryHttpParams<T> {
	args?: Record<string, unknown>
	fn: (args?: Record<string, unknown>) => Promise<AxiosResponse<T>>
}

export const tryHttp = async <T>(
	props: TryHttpParams<T>
): Promise<ApiResponse<T>> => {
	try {
		const response = await props.fn(props.args)
		return response.data as ApiResponse<T>
	} catch (error: unknown) {
		if (axios.isAxiosError(error) && error.response?.data) {
			throw new Error(error.response.data)
		} else {
			throw new Error(JSON.stringify(UknownError))
		}
	}
}
