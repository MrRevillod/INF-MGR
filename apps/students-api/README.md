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
just students-api-run       # Ejecutar API local
```

### Docker
```bash
just students-api-build     # Construir imagen
docker compose up students_api_dev    # Levantar contenedor
docker compose down         # Parar contenedor
```

### Acceso
- **Local**: http://localhost:8000
- **Docker**: http://localhost:8001
