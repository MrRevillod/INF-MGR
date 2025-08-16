FROM rust:1.88-bullseye

WORKDIR /app

RUN cargo install cargo-watch

RUN cargo install --git https://github.com/typst/typst --locked typst-cli

COPY Cargo.toml Cargo.lock ./

COPY apps/server/ ./apps/server/

COPY tests/src ./tests/src
COPY tests/Cargo.toml ./tests/

# Create documents directory
RUN mkdir -p /app/documents

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cd tests/ && cargo watch -x 'test -- --nocapture --test-threads=1' -w src -w /../apps/server/src"
