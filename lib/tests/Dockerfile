FROM rust:1.88-bullseye

WORKDIR /app

RUN cargo install cargo-watch

# Install typst for PDF generation
RUN cargo install --git https://github.com/typst/typst --locked typst-cli

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy lib dependencies
COPY lib/services/ ./lib/services/

# Copy server (dependency for tests)
COPY apps/server/ ./apps/server/

# Copy tests
COPY lib/tests/src ./lib/tests/src
COPY lib/tests/Cargo.toml ./lib/tests/

# Create documents directory
RUN mkdir -p /app/documents

ENV CARGO_TARGET_DIR /app/target

EXPOSE 8000

CMD sh -c "cd lib/tests/ && cargo watch -x 'test -- --nocapture --test-threads=1' -w src -w ../services/src -w ../../apps/server/src"
