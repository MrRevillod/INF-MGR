# Etapa de instalación (común para ambos entornos)
FROM node:lts-alpine AS base
WORKDIR /app
COPY apps/web/package*.json ./
RUN npm install
COPY . .

# Etapa de construcción de la aplicación
FROM base AS builder
WORKDIR /app
COPY --from=base /app .
RUN npm run build

# Etapa de desarrollo
FROM node:lts-alpine AS dev
WORKDIR /app
COPY --from=base /app/node_modules ./node_modules
COPY --from=base /app/package.json ./package.json
COPY . .
CMD ["npm", "run", "dev"]

# Etapa de producción
FROM nginx:alpine AS prod
COPY --from=builder /app/dist /usr/share/nginx/html
COPY config/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]