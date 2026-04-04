#!/usr/bin/env bash

set -e

echo "Configurando conexión WireGuard..."

if ! /app/apps/server/config/vpn.sh; then
    echo "Error al configurar VPN, continuando sin ella..."
else
    echo "VPN configurada exitosamente"
fi

echo "Iniciando..."
cd apps/server && cargo watch -x 'run --bin server' -w src -w config -w tools -w ../../packages/services/src -w ../../packages/tex_parser/src