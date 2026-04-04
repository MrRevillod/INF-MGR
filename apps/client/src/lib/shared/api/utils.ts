import axios, { type AxiosResponse } from "axios"

type RequestArgs = Record<string, unknown> | undefined
type Request<T> = (args?: Record<string, unknown>) => Promise<AxiosResponse<T>>
type Response<T> = Promise<ApiResponse<T>>

/**
 * Makes an API request and handles errors.
 * @param fn - The request function to execute.
 * @param args - Optional arguments for the request function.
 * @Generics
 * T - The expected type of the response "data" field.
 */
export const TryFn = async <T>(fn: Request<T>, args?: RequestArgs): Response<T> => {
	try {
		return (await fn(args)).data as ApiResponse<T>
	} catch (error: unknown) {
		if (axios.isAxiosError(error) && error.response?.data) {
			throw new Error(error.response.data)
		} else {
			throw new Error(JSON.stringify(UknownError))
		}
	}
}

export type ApiResponse<T = unknown> = {
	data?: T
	status: number
	success: boolean
	timestamp: string
	message: string
	error?: string
	errors?: Record<string, ProblemDetails>
}

type ProblemDetails = Array<{
	code?: string
	message: string
}>

export const UknownError = {
	data: null,
	status: 500,
	timestamp: new Date().toISOString(),
	message: "Error desconocido, por favor intente más tarde.",
}
