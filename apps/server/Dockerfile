# -----------------------------------------------------------------------
# BUILDER STAGE ---------------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.90-bookworm AS builder

WORKDIR /app

RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

RUN apt-get update && apt-get install -y \
    unzip \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

COPY apps/server/Cargo.toml apps/server/Cargo.lock ./apps/server/
COPY packages/services/Cargo.toml services/Cargo.lock ./packages/services/
COPY packages/tex_parser/Cargo.toml packages/tex_parser/Cargo.lock ./packages/tex_parser/

RUN mkdir -p ./apps/server/src && echo 'fn main() { println!("Caching dependencies"); }' > ./apps/server/src/main.rs
RUN mkdir -p ./packages/services/src && echo 'pub fn placeholder() {}' > ./packages/services/src/lib.rs
RUN mkdir -p ./packages/tex_parser/src && echo 'pub fn placeholder() {}' > ./packages/tex_parser/src/lib.rs

RUN cd apps/server && cargo build --release
RUN cd packages/services && cargo build --release
RUN cd packages/tex_parser && cargo build --release

RUN rm -rf ./tests/src ./apps/server/src ./apps/server/target/release/deps/server*
RUN rm -rf ./services/src ./services/target/release/deps/services*
RUN rm -rf ./packages/tex_parser/src ./packages/tex_parser/target/release/deps/tex_parser*

COPY apps/server/src ./apps/server/src
COPY apps/server/tools ./apps/server/tools
COPY apps/server/config ./apps/server/config

COPY packages/services/src ./packages/services/src
COPY packages/tex_parser/src ./packages/tex_parser/src

RUN cd apps/server && cargo build --release
RUN cd packages/services && cargo build --release
RUN cd packages/tex_parser && cargo build --release

# -----------------------------------------------------------------------
# DEVELOPMENT STAGE -----------------------------------------------------
# -----------------------------------------------------------------------

FROM rust:1.90-bookworm AS dev

RUN wget https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz \
    && tar -xf typst-x86_64-unknown-linux-musl.tar.xz \
    && mv typst-x86_64-unknown-linux-musl/typst /usr/local/bin/ \
    && rm -rf typst-x86_64-unknown-linux-musl*

RUN apt-get update && apt-get install -y \
    unzip \
    build-essential \
    pkg-config \
    wireguard \
    iproute2 \
    wireguard-tools \
    netcat-traditional \
    iputils-ping \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY packages/services ./packages/services
COPY packages/tex_parser ./packages/tex_parser
COPY apps/server ./apps/server

ENV CARGO_TARGET_DIR /app/target

RUN chmod +x /app/apps/server/config/vpn.sh && \
    chmod +x /app/apps/server/config/entrypoint.sh

EXPOSE 8000

CMD ["/bin/bash", "/app/apps/server/config/entrypoint.sh"]

# -----------------------------------------------------------------------
# PRODUCTION STAGE ------------------------------------------------------
# -----------------------------------------------------------------------

FROM debian:bookworm-slim AS prod

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/bin/unzip /usr/bin/unzip
COPY --from=builder /usr/local/bin/typst /usr/local/bin/typst
COPY --from=builder /app/apps/server/target/release/server /usr/local/bin/server

EXPOSE 8000

CMD ["/usr/local/bin/server"]