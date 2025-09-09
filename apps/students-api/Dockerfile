# Imagen base con UV
FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim

# Directorio de trabajo
WORKDIR /app

# Copiar archivos de configuración
COPY pyproject.toml uv.lock* ./

# Instalar dependencias de desarrollo
RUN uv sync --frozen --extra dev

# Copiar código fuente
COPY . .

# Exponer puerto
EXPOSE 8000

# Comando por defecto
CMD ["uv", "run", "python", "main.py"]