FROM rust:1.88-bullseye

WORKDIR /app

RUN cargo install cargo-watch

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy lib dependencies
COPY lib/services/ ./lib/services/

# Copy server (dependency for tests)
COPY apps/server/ ./apps/server/

# Copy tests
COPY lib/tests/src ./lib/tests/src
COPY lib/tests/Cargo.toml ./lib/tests/

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cd lib/tests/ && cargo watch -x 'test -- --nocapture --test-threads=1' -w src -w ../services/src -w ../../apps/server/src"
