// hooks/useForm.ts
import { getContext, setContext } from "svelte"
import { safeParseAsync, type ObjectSchema } from "valibot"

interface UseFormOptions<T = Record<string, any>> {
	initialData: T
	schema: ObjectSchema<any, any>
	onSubmit: (data: T, changedFields: Partial<T>) => void | Promise<void>
	onValidationError?: (errors: Record<string, string>) => void
	mode?: "create" | "update" // Nuevo: especifica el modo del formulario
	validateOnlyChanged?: boolean // Nuevo: si validar solo campos cambiados (default: true para update, false para create)
}

interface FormState<T = Record<string, any>> {
	data: T
	errors: Record<string, string>
	isSubmitting: boolean
	isDirty: boolean
}

interface FormActions<T = Record<string, any>> {
	updateField: (name: keyof T, value: any) => void
	setFieldError: (name: keyof T, error: string) => void
	clearFieldError: (name: keyof T) => void
	clearAllErrors: () => void
	reset: () => void
	submit: () => Promise<void>
	getChangedFields: () => Partial<T>
}

interface FormHook<T = Record<string, any>> {
	state: FormState<T>
	actions: FormActions<T>
	createContext: (key: string) => void
	field: (name: keyof T) => FieldHook<T[keyof T]>
}

interface FieldHook<V = any> {
	value: V
	error: string | undefined
	isDirty: boolean
	update: (value: V) => void
	clearError: () => void
}

export function useForm<T extends Record<string, any> = Record<string, any>>(
	options: UseFormOptions<T>
): FormHook<T> {
	const {
		initialData,
		schema,
		onSubmit,
		onValidationError,
		mode = "update",
		validateOnlyChanged = mode === "update",
	} = options

	// Estado reactivo
	let data = $state({ ...initialData } as T)
	let errors = $state<Record<string, string>>({})
	let isSubmitting = $state(false)
	let originalData = $state({ ...initialData } as T)

	$inspect("initialData", initialData) // Para depuración

	// Efecto para actualizar cuando initialData cambie (solo en modo update)
	$effect(() => {
		if (mode === "create") return // No actualizar automáticamente en modo create

		const hasValidData = Object.values(initialData).some(
			value => value !== undefined && value !== null && value !== ""
		)

		if (hasValidData) {
			// Solo actualizar campos que no han sido modificados por el usuario
			for (const key in initialData) {
				if (
					data[key] === originalData[key] ||
					data[key] === undefined ||
					data[key] === null
				) {
					data[key] = initialData[key]
				}
			}
			originalData = { ...initialData } as T
		}
	})

	// Estado computado
	const isDirty = $derived(
		Object.keys(data).some(key => data[key] !== originalData[key])
	)

	const getChangedFields = (): Partial<T> => {
		const changed: Partial<T> = {}
		for (const key in data) {
			if (data[key] !== originalData[key]) {
				changed[key] = data[key]
			}
		}
		return changed
	}

	// Acciones
	const updateField = (name: keyof T, value: any) => {
		data[name] = value
		// Limpiar error del campo cuando se actualiza
		if (errors[name as string]) {
			delete errors[name as string]
			errors = { ...errors }
		}
	}

	const setFieldError = (name: keyof T, error: string) => {
		errors[name as string] = error
		errors = { ...errors }
	}

	const clearFieldError = (name: keyof T) => {
		if (errors[name as string]) {
			delete errors[name as string]
			errors = { ...errors }
		}
	}

	const clearAllErrors = () => {
		errors = {}
	}

	const reset = () => {
		data = { ...originalData } as T
		errors = {}
	}

	const submit = async () => {
		if (isSubmitting) return

		isSubmitting = true
		clearAllErrors()

		try {
			const changedFields = getChangedFields()

			// Determinar qué datos validar según el modo
			const dataToValidate = validateOnlyChanged ? changedFields : data

			// En modo create, validar todos los datos; en update, solo los cambiados (si validateOnlyChanged es true)
			const validationResult = await safeParseAsync(schema, dataToValidate)

			if (!validationResult.success) {
				const validationErrors: Record<string, string> = {}
				for (const error of validationResult.issues) {
					const fieldName = error.path?.[0] as string
					if (fieldName) {
						validationErrors[fieldName] = error.message
					}
				}
				errors = validationErrors
				onValidationError?.(validationErrors)
				return
			}

			await onSubmit(data, changedFields)

			// En modo create, limpiar el formulario después del submit exitoso
			if (mode === "create") {
				data = { ...initialData } as T
				originalData = { ...initialData } as T
			} else {
				// En modo update, actualizar originalData después de un submit exitoso
				originalData = { ...data } as T
			}
		} catch (error) {
			console.error("Form submission error:", error)
		} finally {
			isSubmitting = false
		}
	}

	// Método para crear contexto
	const createContext = (key: string) => {
		const contextValue = $derived({
			data,
			errors,
			isSubmitting,
			isDirty,
			updateField,
			setFieldError,
			clearFieldError,
			clearAllErrors,
			reset,
			submit,
			getChangedFields,
			field, // Agregar el método field al contexto
		})

		setContext(key, contextValue)
	}

	// Helper para trabajar con campos individuales
	const field = (name: keyof T): FieldHook<T[keyof T]> => {
		return {
			get value() {
				return data[name]
			},
			get error() {
				return errors[name as string]
			},
			get isDirty() {
				return data[name] !== originalData[name]
			},
			update: (value: T[keyof T]) => updateField(name, value),
			clearError: () => clearFieldError(name),
		}
	}

	return {
		state: {
			get data() {
				return data
			},
			get errors() {
				return errors
			},
			get isSubmitting() {
				return isSubmitting
			},
			get isDirty() {
				return isDirty
			},
		},
		actions: {
			updateField,
			setFieldError,
			clearFieldError,
			clearAllErrors,
			reset,
			submit,
			getChangedFields,
		},
		createContext,
		field,
	}
}

// Hook para consumir el contexto del formulario
export function useFormContext<T extends Record<string, any> = Record<string, any>>(
	key: string
) {
	const context = getContext(key)
	if (!context) {
		throw new Error(
			`Form context "${key}" not found. Make sure to call createContext() first.`
		)
	}
	return context as ReturnType<typeof useForm<T>>["state"] &
		ReturnType<typeof useForm<T>>["actions"]
}
