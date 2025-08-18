# Plataforma de Gestión de Prácticas Universitarias

La carrera de Ingeniería Civil en Informática requiere una plataforma web para gestionar prácticas iniciales y profesionales. El sistema busca centralizar la comunicación, gestión documental y seguimiento de procesos en un solo lugar.

---

## Sistema actual
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

## Mejora planteada
Se propone una **plataforma web con perfiles de acceso**: estudiante, profesor, secretaría y administración.

### Nuevo flujo
1. **Creación de perfiles:** Secretaría carga CSV con inscritos; el sistema genera usuarios y credenciales.  
2. **Profesor integrado:** Acceso directo a informes y documentos.  
3. **Bitácoras eliminadas por correo:** Solo se anexan al informe final.  
4. **Registro de empresa automatizado:** Notificación y autorización online.  
5. **Entrega de informe final:** Subida en PDF con opción de enlace online.  
6. **Cálculo automático y cierre:** Promedios y estados gestionados por el sistema.  
7. **Historial permanente:** Informes y evaluaciones quedan almacenados indefinidamente.

### Ventajas
- Automatización de tareas.  
- Comunicación eficiente sin cadenas de correos.  
- Trazabilidad y auditoría.  
- Seguridad en el acceso por roles.  
- Historial disponible siempre.  
- Diseño modular y escalable.  
- Interfaz clara y adaptada a cada perfil.  

---

## Apartado técnico

### Arquitectura
- **Cliente-servidor.**  
- **PostgreSQL** (persistencia).  
- **Redis** (sesiones y caché).  
- Separación de capas (persistencia, negocio, red).

### Tecnologías
- **Cliente:** SvelteKit (rápido y ligero).  
- **Servidor:** Rust (seguro y sin garbage collector).  
- **IA/LLM:** Evaluaciones automáticas basadas en rúbricas vía protocolo MCP.  
- **Base de datos vectorial:** Detección de similitudes y plagio en informes.

---

## Conclusión
El sistema actual funciona, pero es lento y burocrático.  
La nueva plataforma reduce la dependencia del correo, automatiza procesos y ofrece un flujo más claro y rápido. Con **SvelteKit + Rust**, el sistema será eficiente y estable, incorporando además IA y almacenamiento vectorial para mantener la integridad académica.  
El resultado esperado: **menos carga para administrativos y profesores, más claridad para estudiantes.**
