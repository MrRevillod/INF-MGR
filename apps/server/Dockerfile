# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.90-bullseye AS builder

WORKDIR /app

RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

COPY Cargo.toml Cargo.lock ./
COPY apps/server/Cargo.toml apps/server/Cargo.lock ./apps/server/
COPY services/Cargo.toml services/Cargo.lock ./services/

RUN mkdir -p ./apps/server/src && echo 'fn main() { println!("Caching dependencies"); }' > ./apps/server/src/main.rs
RUN mkdir -p ./services/src && echo 'pub fn placeholder() {}' > ./services/src/lib.rs

RUN cd apps/server && cargo build --release
RUN cd services && cargo build --release
RUN rm -rf ./tests/src ./apps/server/src ./apps/server/target/release/deps/server*
RUN rm -rf ./services/src ./services/target/release/deps/services*

COPY apps/server/src ./apps/server/src
COPY apps/server/tools ./apps/server/tools
COPY apps/server/config ./apps/server/config

COPY services/src ./services/src

RUN cd apps/server && cargo build --release
RUN cd services && cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.89-bullseye AS dev

RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY services ./services
COPY apps/server ./apps/server

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cd apps/server && cargo watch -x 'run --bin server' -w src -w config -w tools -w ../../services/src"

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bullseye-slim AS prod

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/typst /usr/local/bin/typst
COPY --from=builder /app/apps/server/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["/usr/local/bin/server"]