# Imagen base con UV
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS base

# Directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración
COPY apps/students-api/pyproject.toml apps/students-api/uv.lock* ./

# Instalar dependencias de desarrollo
RUN uv sync --frozen 

# Target DESARROLLO (con watch/reload)
FROM base AS dev
COPY apps/students-api/ .
EXPOSE 7000
CMD ["uv", "run", "uvicorn", "src.students_api.app:app", "--host", "0.0.0.0", "--port", "7000", "--reload"]

# Target PRODUCCIÓN (con workers)
FROM base AS prod
COPY apps/students-api/ .
EXPOSE 7000
CMD ["uv", "run", "uvicorn", "src.students_api.app:app", "--host", "0.0.0.0", "--port", "7000", "--workers", "4"]