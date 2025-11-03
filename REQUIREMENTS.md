# Plataforma de Gestión de Prácticas Universitarias

La carrera de Ingeniería Civil en Informática requiere una plataforma web para gestionar prácticas iniciales y profesionales. El sistema busca centralizar la comunicación, gestión documental y seguimiento de procesos en un solo lugar.

---

## Problema actual
Actualmente el proceso depende de **correo electrónico** y de un **coordinador de prácticas** que:
1. Inscribe al estudiante en el curso.
2. Asigna un profesor responsable.
3. Realiza una inducción y entrega documentos (autorización, bitácora, pautas).
4. Recibe documentos iniciales y coordina entregas.
5. Supervisa bitácoras semanales.
6. Revisa informe final.
7. Evalúa junto a la empresa.
8. Entrega nota final al profesor encargado.

**Problemas:**  
- Dependencia del correo electrónico.  
- Exceso de gestión manual.  
- Cargo administrativo exclusivo.  

---

## Mejora planteada - Requerimientos de la plataforma
Se propone una **plataforma web con perfiles de acceso**: estudiante, profesor, secretaría y administración.

### Funcionalidades clave

La web se encontrará desarrollada utilizando sveltekit + svelte5 (runas). con tanstack svelte query (https://tanstack.com/query/v5/docs/framework/svelte/overview) para la gestión de datos asincrónicos. Valibot para la validación de formularios (https://valibot.dev/), Formisch para el manejo de formularios (https://formisch.dev/) y Tailwind CSS para el diseño visual (https://tailwindcss.com/).

-  **Profesores**: Pueden tener múltiples cursos asignados, gestionar estudiantes del curso, visualizar información relevante, inscribir prácticas a estudiantes.

- **Estudiantes**: Acceden a su información, documentos y estado de prácticas, subir su informe de prácticas. Visualizar sus cursos.

- **Secretaría**: Gestiona cursos, asigna profesores, supervisa estado de prácticas, genera reportes.

- **Administración**: Gestiona usuarios, roles y permisos, supervisa la plataforma, crear cursos.
