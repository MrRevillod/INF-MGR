export type UserRole =
	| "administrator"
	| "teacher"
	| "student"
	| "coordinator"
	| "secretary"

export interface User {
	id: string
	rut: string
	name: string
	email: string
	roles: UserRole[]
}
