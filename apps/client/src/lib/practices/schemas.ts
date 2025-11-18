import * as v from "valibot"

export interface Practice {
	id: string
	enterpriseName: string
	location: string
	description: string
	supervisorName: string
	supervisorEmail: string
	supervisorPhone: string
	startDate: string | null
	endDate: string | null
	practiceStatus: PracticeStatus
}

export const PracticeStatusSchema = v.union([
	v.literal("pending"),
	v.literal("approved"),
	v.literal("declined"),
])

export type PracticeStatus = v.InferInput<typeof PracticeStatusSchema>

// Schema para crear práctica
export const CreatePracticeSchema = v.pipe(
	v.object({
		enterpriseName: v.pipe(
			v.string(),
			v.minLength(1, "El nombre de la empresa es requerido"),
			v.maxLength(255, "El nombre no puede exceder 255 caracteres")
		),
		description: v.pipe(
			v.string(),
			v.minLength(1, "La descripción es requerida"),
			v.maxLength(255, "La descripción no puede exceder 255 caracteres")
		),
		location: v.pipe(
			v.string(),
			v.minLength(1, "La ubicación es requerida"),
			v.maxLength(255, "La ubicación no puede exceder 255 caracteres")
		),
		supervisorName: v.pipe(
			v.string(),
			v.minLength(1, "El nombre del supervisor es requerido"),
			v.maxLength(100, "El nombre no puede exceder 100 caracteres")
		),
		supervisorEmail: v.pipe(
			v.string(),
			v.minLength(1, "El email del supervisor es requerido"),
			v.email("El email debe ser válido")
		),
		supervisorPhone: v.pipe(
			v.string(),
			v.minLength(1, "El teléfono es requerido"),
			v.regex(
				/^(\+?56)?(\s?)(0?9)(\s?)[9876543]\d{7}$/,
				"El teléfono debe ser un número chileno válido (ej: +56912345678)"
			)
		),
		startDate: v.pipe(v.string(), v.isoDateTime("Fecha de inicio inválida")),
		endDate: v.pipe(v.string(), v.isoDateTime("Fecha de término inválida")),
	}),
	v.check(data => {
		const start = new Date(data.startDate)
		const end = new Date(data.endDate)
		return end > start
	}, "La fecha de término debe ser posterior a la fecha de inicio")
)

export type CreatePractice = v.InferInput<typeof CreatePracticeSchema>

// Schema para actualizar práctica
export const UpdatePracticeSchema = v.pipe(
	v.object({
		enterpriseName: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "El nombre de la empresa es requerido"),
				v.maxLength(255, "El nombre no puede exceder 255 caracteres")
			)
		),
		description: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "La descripción es requerida"),
				v.maxLength(255, "La descripción no puede exceder 255 caracteres")
			)
		),
		location: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "La ubicación es requerida"),
				v.maxLength(255, "La ubicación no puede exceder 255 caracteres")
			)
		),
		supervisorName: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "El nombre del supervisor es requerido"),
				v.maxLength(100, "El nombre no puede exceder 100 caracteres")
			)
		),
		supervisorEmail: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "El email del supervisor es requerido"),
				v.email("El email debe ser válido")
			)
		),
		supervisorPhone: v.optional(
			v.pipe(
				v.string(),
				v.minLength(1, "El teléfono es requerido"),
				v.regex(
					/^(\+?56)?(\s?)(0?9)(\s?)[9876543]\d{7}$/,
					"El teléfono debe ser un número chileno válido"
				)
			)
		),
		startDate: v.optional(v.pipe(v.string(), v.isoDateTime("Fecha inválida"))),
		endDate: v.optional(v.pipe(v.string(), v.isoDateTime("Fecha inválida"))),
	}),
	v.check(data => {
		if (data.startDate && data.endDate) {
			const start = new Date(data.startDate)
			const end = new Date(data.endDate)
			return end > start
		}
		return true
	}, "La fecha de término debe ser posterior a la fecha de inicio")
)

export type UpdatePractice = v.InferInput<typeof UpdatePracticeSchema>

// Schema para evaluar práctica
export const EvaluatePracticeSchema = v.object({
	score: v.pipe(
		v.number(),
		v.minValue(1, "La nota debe ser al menos 1.0"),
		v.maxValue(7, "La nota no puede exceder 7.0")
	),
})

export type EvaluatePractice = v.InferInput<typeof EvaluatePracticeSchema>
