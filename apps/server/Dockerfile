# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS builder

WORKDIR /app

# Build dependencies

COPY apps/server/Cargo.toml apps/server/Cargo.lock ./

RUN mkdir -p ./src && echo 'fn main() { println!("Caching dependencies"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src ./target/release/deps/server*

# Build entire application

COPY apps/server/src ./src
RUN cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.84-bullseye AS dev

RUN cargo install cargo-watch

WORKDIR /app

COPY apps/server/Cargo.toml apps/server/Cargo.lock ./
COPY apps/server/src ./src/
COPY apps/server/tools ./tools/

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "echo 'INFO: Attempting to run database seeder...' && \
           cargo run --bin dev-seeder --features seeder && \
           echo 'INFO: Database seeder finished successfully.' && \
           echo 'INFO: Starting server with cargo watch...' && \
           cargo watch -x 'run --bin server' || \
           (echo 'ERROR: Database seeder failed. Check logs above.' && exit 1)"

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS prod

COPY --from=builder /app/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["/usr/local/bin/server"]