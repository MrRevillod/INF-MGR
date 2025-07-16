run:
    docker compose up

lint:
    cargo clippy --all-features -- -D warnings

fmt:
    cargo fmt --all -- --check

db-seed:
    docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder
