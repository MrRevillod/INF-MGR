# Etapa de instalación (común para ambos entornos)

FROM node:lts-alpine AS base
WORKDIR /app
COPY apps/client/package*.json ./
RUN npm install
COPY apps/client .

# Etapa de construcción de la aplicación

FROM base AS builder
WORKDIR /app
RUN npm run build

# Etapa de desarrollo

FROM node:lts-alpine AS dev
WORKDIR /app
COPY apps/client/package*.json ./
RUN npm install
COPY apps/client .
EXPOSE 5173
CMD ["npm", "run", "dev"]

# Etapa de producción

FROM nginx:alpine AS prod
COPY --from=builder /app/build /usr/share/nginx/html
COPY config/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]