# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.89-bullseye AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src
COPY config ./config

RUN cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.89-bullseye AS dev

WORKDIR /app

RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS production

WORKDIR /app

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/students-api /usr/local/bin/students-api

COPY --from=builder /app/config ./config

RUN groupadd -r appuser && useradd -r -g appuser appuser
RUN chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["students-api"]