# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.88-bullseye AS builder

WORKDIR /app

# Instalar Typst v0.13.1 en el builder stage
RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

# Build dependencies

COPY Cargo.toml Cargo.lock ./
COPY lib/services/Cargo.toml ./lib/services/
COPY lib/tests/Cargo.toml ./lib/tests/
COPY apps/server/Cargo.toml apps/server/Cargo.lock ./apps/server/

# Create dummy files for dependency caching
RUN mkdir -p ./lib/services/src && echo 'fn main() {}' > ./lib/services/src/lib.rs
RUN mkdir -p ./lib/tests/src && echo 'fn main() {}' > ./lib/tests/src/lib.rs
RUN mkdir -p ./apps/server/src && echo 'fn main() { println!("Caching dependencies"); }' > ./apps/server/src/main.rs

RUN cd apps/server && cargo build --release
RUN rm -rf ./lib/services/src ./lib/tests/src ./apps/server/src ./apps/server/target/release/deps/server*

# Build entire application

COPY lib/services/src ./lib/services/src
COPY apps/server/src ./apps/server/src
COPY apps/server/tools ./apps/server/tools
COPY apps/server/config ./apps/server/config

RUN cd apps/server && cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.88-bullseye AS dev

# Instalar Typst v0.13.1 para desarrollo
RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY lib/services ./lib/services
COPY apps/server ./apps/server
COPY lib/tests ./lib/tests

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cd apps/server && cargo watch -x 'run --bin server' -w src -w config -w tools -w ../../lib/services/src"

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS prod

# Instalar dependencias necesarias para Typst
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar Typst del builder stage
COPY --from=builder /usr/local/bin/typst /usr/local/bin/typst

COPY --from=builder /app/apps/server/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["/usr/local/bin/server"]