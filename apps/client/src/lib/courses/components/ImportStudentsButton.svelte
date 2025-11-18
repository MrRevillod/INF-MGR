<script lang="ts">
	import Modal from "$lib/shared/components/Modal.svelte"
	import Button from "$lib/shared/components/ui/Button.svelte"
	import { importStudentsMutation } from "$lib/courses/mutations"
	import { ImportStudentsSchema, type ImportedStudent } from "$lib/courses/schemas"
	import * as v from "valibot"

	interface Props {
		courseId: string
	}

	let { courseId }: Props = $props()

	let showModal = $state(false)
	let csvFile = $state<File | null>(null)
	let students = $state<ImportedStudent[]>([])
	let parseError = $state<string>("")
	let validationErrors = $state<string[]>([])

	const mutation = $derived(importStudentsMutation(courseId))

	function handleFileChange(event: Event) {
		const target = event.target as HTMLInputElement
		const file = target.files?.[0]

		if (!file) return

		if (!file.name.endsWith(".csv")) {
			parseError = "El archivo debe ser un CSV"
			csvFile = null
			students = []
			return
		}

		csvFile = file
		parseError = ""
		validationErrors = []

		// Leer y parsear el CSV
		const reader = new FileReader()
		reader.onload = e => {
			try {
				const text = e.target?.result as string
				const parsed = parseCSV(text)
				students = parsed
			} catch (error) {
				parseError = error instanceof Error ? error.message : "Error al parsear CSV"
				students = []
			}
		}
		reader.readAsText(file)
	}

	function parseCSV(text: string): ImportedStudent[] {
		const lines = text.trim().split("\n")

		if (lines.length < 2) {
			throw new Error(
				"El CSV debe tener al menos una fila de encabezados y una de datos"
			)
		}

		// Verificar encabezados (primera línea)
		const headers = lines[0].split(",").map(h => h.trim().toLowerCase())
		const requiredHeaders = ["rut", "name", "email", "register"]

		const missingHeaders = requiredHeaders.filter(h => !headers.includes(h))
		if (missingHeaders.length > 0) {
			throw new Error(
				`Faltan columnas requeridas: ${missingHeaders.join(", ")}. Las columnas deben ser: rut, name, email, register`
			)
		}

		// Parsear filas de datos
		const parsedStudents: ImportedStudent[] = []

		for (let i = 1; i < lines.length; i++) {
			const line = lines[i].trim()
			if (!line) continue // Saltar líneas vacías

			const values = line.split(",").map(v => v.trim())

			if (values.length !== headers.length) {
				throw new Error(
					`Fila ${i + 1}: número incorrecto de columnas (esperadas ${headers.length}, encontradas ${values.length})`
				)
			}

			const student: any = {}
			headers.forEach((header, index) => {
				student[header] = values[index]
			})

			// Normalizar el RUT: quitar puntos y convertir DV a mayúscula
			// Ejemplos: "12.345.678-9" -> "12345678-9", "12345678-k" -> "12345678-K"
			const cleanRut = student.rut
				? student.rut.replace(/\./g, "").toUpperCase()
				: ""

			parsedStudents.push({
				rut: cleanRut,
				name: student.name || "",
				email: student.email || "",
				register: student.register || "",
			})
		}

		return parsedStudents
	}

	async function handleImport() {
		validationErrors = []

		// Validar con el schema
		const result = v.safeParse(ImportStudentsSchema, { students })

		if (!result.success) {
			validationErrors = result.issues.map(issue => {
				const path = issue.path?.map(p => p.key).join(".") || "general"
				return `${path}: ${issue.message}`
			})
			return
		}

		try {
			await mutation.mutateAsync({ students })
			showModal = false
			csvFile = null
			students = []
			parseError = ""
			validationErrors = []
		} catch (error) {
			console.error("Error importing students:", error)
		}
	}

	function handleCancel() {
		showModal = false
		csvFile = null
		students = []
		parseError = ""
		validationErrors = []
	}
</script>

<Button
	onclick={() => (showModal = true)}
	variant="primary"
	text="📊 Importar desde Excel/CSV"
/>

<Modal
	bind:isOpen={showModal}
	onClose={handleCancel}
	title="Importar Estudiantes desde CSV"
>
	<div class="space-y-6">
		<!-- Instrucciones -->
		<div class="rounded-lg border border-blue-200 bg-blue-50 p-4">
			<h3 class="mb-2 text-sm font-semibold text-blue-900">
				Formato del archivo CSV
			</h3>
			<p class="mb-2 text-sm text-blue-800">
				El archivo debe tener las siguientes columnas (en la primera fila):
			</p>
			<ul class="list-inside list-disc space-y-1 text-sm text-blue-800">
				<li>
					<strong>rut</strong> - Formato: 12.345.678-9 o 12345678-9 (los puntos se quitan
					automáticamente)
				</li>
				<li>
					<strong>name</strong> - Nombre completo del estudiante (mínimo 5 caracteres)
				</li>
				<li><strong>email</strong> - Email válido</li>
				<li><strong>register</strong> - Número de matrícula</li>
			</ul>
			<p class="mt-2 text-xs text-blue-700">
				Ejemplo de header: rut,name,email,register
			</p>
		</div>

		<!-- Input de archivo -->
		<div>
			<label for="csv-file" class="mb-2 block text-sm font-medium text-gray-700">
				Seleccionar archivo CSV
			</label>
			<input
				id="csv-file"
				type="file"
				accept=".csv"
				onchange={handleFileChange}
				class="block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
			/>
		</div>

		<!-- Errores de parseo -->
		{#if parseError}
			<div class="rounded-lg border border-red-200 bg-red-50 p-4">
				<p class="text-sm font-semibold text-red-900">Error al leer el archivo:</p>
				<p class="text-sm text-red-800">{parseError}</p>
			</div>
		{/if}

		<!-- Errores de validación -->
		{#if validationErrors.length > 0}
			<div class="rounded-lg border border-red-200 bg-red-50 p-4">
				<p class="mb-2 text-sm font-semibold text-red-900">Errores de validación:</p>
				<ul class="list-inside list-disc space-y-1 text-sm text-red-800">
					{#each validationErrors as error}
						<li>{error}</li>
					{/each}
				</ul>
			</div>
		{/if}

		<!-- Preview de estudiantes -->
		{#if students.length > 0}
			<div class="rounded-lg border border-gray-200 bg-white">
				<div class="border-b border-gray-200 px-4 py-3">
					<h3 class="text-sm font-semibold text-gray-900">
						Vista previa ({students.length} estudiante{students.length !== 1
							? "s"
							: ""})
					</h3>
				</div>
				<div class="max-h-64 overflow-y-auto">
					<table class="min-w-full divide-y divide-gray-200">
						<thead class="sticky top-0 bg-gray-50">
							<tr>
								<th
									class="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500"
								>
									RUT
								</th>
								<th
									class="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500"
								>
									Nombre
								</th>
								<th
									class="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500"
								>
									Email
								</th>
								<th
									class="px-4 py-2 text-left text-xs font-medium uppercase text-gray-500"
								>
									Matrícula
								</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-200 bg-white">
							{#each students as student, index (index)}
								<tr class="hover:bg-gray-50">
									<td class="whitespace-nowrap px-4 py-2 text-sm text-gray-900">
										{student.rut}
									</td>
									<td class="px-4 py-2 text-sm text-gray-900">
										{student.name}
									</td>
									<td class="px-4 py-2 text-sm text-gray-900">
										{student.email}
									</td>
									<td class="px-4 py-2 text-sm text-gray-900">
										{student.register}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}

		<!-- Botones de acción -->
		<div class="flex justify-end gap-3">
			<Button onclick={handleCancel} variant="secondary" text="Cancelar" />
			<Button
				onclick={handleImport}
				variant="primary"
				text={mutation.isPending ? "Importando..." : "Importar Estudiantes"}
				disabled={students.length === 0 || mutation.isPending}
			/>
		</div>
	</div>
</Modal>
