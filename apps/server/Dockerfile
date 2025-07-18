# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.88-bullseye AS builder

WORKDIR /app

# Build dependencies

COPY apps/server/Cargo.toml apps/server/Cargo.lock ./

RUN mkdir -p ./src && echo 'fn main() { println!("Caching dependencies"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src ./target/release/deps/server*

# Build entire application

COPY apps/server/src ./src
COPY apps/server/html ./html
RUN cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.88-bullseye AS dev

RUN cargo install cargo-watch

WORKDIR /app

COPY apps/server/config ./config
COPY apps/server/html ./html
COPY apps/server/Cargo.toml apps/server/Cargo.lock ./
COPY apps/server/src ./src/
COPY apps/server/tools ./tools/

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cargo watch -x 'run --bin server' -w src -w config"

# -----------------------------------------------------------------------
# TEST STAGE ------------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.88-bullseye AS test

WORKDIR /app

RUN cargo install cargo-watch

COPY apps/server/config ./config
COPY apps/server/html ./html
COPY apps/server/Cargo.toml apps/server/Cargo.lock ./
COPY apps/server/src ./src/
COPY apps/server/tools ./tools/

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cargo watch -x 'test -- --nocapture' -w src -w config"

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS prod

COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /app/html ./html

EXPOSE 8000

CMD ["/usr/local/bin/server"]