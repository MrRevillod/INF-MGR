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
## Probar Ruff
```bash
uv run ruff check . # Verificar código
uv run ruff format . # Formatear código
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
- **Docker**: http://localhost:7001

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

