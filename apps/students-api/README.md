# Students API - Entorno de Desarrollo

## Inicializamos el proyecto 
```bash
uv init --name students-api
```

## Añademos dependencias de desarrollo(Ruff) 
- Una ves añadidas las dependencias al `pyproject.toml`, sincronizamos el entorno con:

```bash
 uv sync 
 ```
## Probar Ruff (Dentro del contenedor Docker)
```bash
# IMPORTANTE: Ejecutar desde el directorio raíz del proyecto
cd /home/benjamin/Proyecto_practicas/INF-MGR

# Verificar código
docker exec inf_mgr_students_api_dev uv run ruff check .

# Formatear código
docker exec inf_mgr_students_api_dev uv run ruff format .

# Arreglar errores automáticamente
docker exec inf_mgr_students_api_dev uv run ruff check --fix .
```
## Herramientas Configuradas
- **Astral UV** - Gestor de paquetes Python
- **Astral Ruff** - Linter + Formatter
- **Docker** - Containerización
- **Justfile** - Comandos integrados

## Comandos

### Desarrollo
```bash
just students-api-lint      # Verificar código
just students-api-format    # Formatear código
just students-api-run       # Ejecutar API 
```

### Docker
```bash
just students-api-build     # Construir imagen
docker compose up students_api_dev    # Levantar contenedor
docker compose down         # Parar contenedor
```

### Acceso
- **Local**: http://localhost:7000
- **Docker**: http://localhost:7000

## API Base con FastAPI
- **FastAPI** - API REST en puerto 7000
- **Perfiles de ejecución** - Desarrollo (watch) y producción (workers)
- **Docker targets** - `dev` y `prod`

### Targets Docker
```bash
# Desarrollo (watch mode)
docker compose up students_api_dev

# Producción (4 workers)  
docker build -t api-prod --target prod ./apps/students-api
docker run -p 7001:7000 api-prod
```
# Ver logs en tiempo real
docker-compose logs -f students_api_dev

# Reiniciar el servicio
docker-compose restart students_api_dev

# Parar el servicio
docker-compose stop students_api_dev

# Ver estado de todos los servicios
docker-compose ps


### Endpoints disponibles
- `/` - Mensaje de bienvenida
- `/health` - Estado de la API


# Ver la API corriendo
curl http://localhost:7000/

# Ver estudiantes
curl http://localhost:7000/students

# Ver cursos  
curl http://localhost:7000/courses

# Ver matrículas
curl http://localhost:7000/enrollments

# Ver estudiantes de un curso específico en un año específico (UCT)
curl -i -H "x-api-key: en.env" http://localhost:7000/uct/courses/{course_id}/{year}/student


# Documentación interactiva
open http://localhost:7000/docs