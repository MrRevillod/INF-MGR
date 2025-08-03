<div class="space-y-8">
	<div class="border-border border-b pb-8">
		<h1 class="text-text-primary text-3xl font-semibold">Bienvenido al Dashboard</h1>
		<p class="text-text-muted mt-3 text-lg">
			Sistema de gestión académica - Panel de administración
		</p>
	</div>

	<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
		<div
			class="bg-surface border-border rounded-lg border p-6 transition-shadow hover:shadow-sm"
		>
			<h3 class="text-text-muted text-sm font-medium uppercase tracking-wide">
				Estudiantes
			</h3>
			<p class="text-text-primary mt-2 text-3xl font-semibold">1,234</p>
		</div>

		<div
			class="bg-surface border-border rounded-lg border p-6 transition-shadow hover:shadow-sm"
		>
			<h3 class="text-text-muted text-sm font-medium uppercase tracking-wide">
				Personal
			</h3>
			<p class="text-text-primary mt-2 text-3xl font-semibold">45</p>
		</div>

		<div
			class="bg-surface border-border rounded-lg border p-6 transition-shadow hover:shadow-sm"
		>
			<h3 class="text-text-muted text-sm font-medium uppercase tracking-wide">
				Asignaturas
			</h3>
			<p class="text-text-primary mt-2 text-3xl font-semibold">89</p>
		</div>

		<div
			class="bg-surface border-border rounded-lg border p-6 transition-shadow hover:shadow-sm"
		>
			<h3 class="text-text-muted text-sm font-medium uppercase tracking-wide">
				Activos
			</h3>
			<p class="text-text-primary mt-2 text-3xl font-semibold">1,189</p>
		</div>
	</div>

	<div class="bg-surface border-border rounded-lg border p-6">
		<h2 class="text-text-primary mb-4 text-lg font-medium">Acciones rápidas</h2>
		<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
			<a
				href="/users/students"
				class="border-border hover:bg-hover-bg flex items-center rounded-lg border p-4 transition-colors"
			>
				<div class="text-sm">
					<h3 class="text-text-primary font-medium">Gestionar Estudiantes</h3>
					<p class="text-text-muted">Ver y administrar estudiantes</p>
				</div>
			</a>

			<a
				href="/users/staff"
				class="border-border hover:bg-hover-bg flex items-center rounded-lg border p-4 transition-colors"
			>
				<div class="text-sm">
					<h3 class="text-text-primary font-medium">Personal Académico</h3>
					<p class="text-text-muted">Administrar personal docente</p>
				</div>
			</a>

			<a
				href="/asignatures"
				class="border-border hover:bg-hover-bg flex items-center rounded-lg border p-4 transition-colors"
			>
				<div class="text-sm">
					<h3 class="text-text-primary font-medium">Asignaturas</h3>
					<p class="text-text-muted">Gestionar materias y cursos</p>
				</div>
			</a>
		</div>
	</div>
</div>
