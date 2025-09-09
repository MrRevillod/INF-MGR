# Imagen base con UV
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS base

# Directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración
COPY pyproject.toml uv.lock* ./

# Instalar dependencias de desarrollo
RUN uv sync --frozen 

# Target DESARROLLO (con watch/reload)
FROM base AS dev
COPY . .
EXPOSE 7000
CMD ["uv", "run", "uvicorn", "src.students_api.app:app", "--host", "0.0.0.0", "--port", "7000", "--reload"]

# Target PRODUCCIÓN (con workers)
FROM base AS prod
COPY . .
EXPOSE 7000
CMD ["uv", "run", "uvicorn", "src.students_api.app:app", "--host", "0.0.0.0", "--port", "7000", "--workers", "4"]