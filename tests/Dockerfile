
FROM rust:1.88-bullseye

WORKDIR /app

RUN cargo install cargo-watch

COPY Cargo.toml Cargo.lock ./
COPY apps/server/ ./apps/server/

COPY tests/src ./tests/src
COPY tests/Cargo.toml ./tests/
COPY apps/server/config ./tests/config
COPY apps/server/tools/templates ./tests/tools/templates

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

# CMD por defecto: ejecuta todos los tests con watch
# Se usa solo con "docker compose up" (just test-all watch)
# Se sobrescribe con "docker compose run" para tests específicos
CMD sh -c "cd tests/ && cargo watch -x 'test -- --nocapture --test-threads=1' -w src"
