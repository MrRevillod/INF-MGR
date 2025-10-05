# Etapa base (instalación de dependencias)
FROM node:lts-alpine AS base

# Instala pnpm globalmente
RUN npm install -g pnpm

WORKDIR /app

# Copia solo los archivos de dependencias para aprovechar cache
COPY apps/client/package.json apps/client/package-lock.json pnpm-lock.yaml ./

# Instala dependencias con pnpm
RUN pnpm install

# Copia el resto del código
COPY apps/client .

# -----------------------------
# Etapa de construcción
FROM base AS builder
WORKDIR /app
RUN pnpm run build

# -----------------------------
# Etapa de desarrollo
FROM node:lts-alpine AS dev
RUN npm install -g pnpm
WORKDIR /app
COPY apps/client/package.json apps/client/package-lock.json pnpm-lock.yaml ./
RUN pnpm install
COPY apps/client .
EXPOSE 5173
CMD ["pnpm", "run", "dev"]

# -----------------------------
# Etapa de producción
FROM nginx:alpine AS prod
COPY --from=builder /app/build /usr/share/nginx/html
COPY config/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
