run DOCKERARGS:
    docker compose up {{DOCKERARGS}}

lint:
    cargo clippy --all-features -- -D warnings && \
    cd apps/web && npm run lint && cd ../..

fmt:
    cargo fmt --all -- --check && \
    cd apps/web && npm run format && cd ../..

db-seed:
    docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder

install:
    cd apps/web && npm install && cd ../..

web-install package:
    cd apps/web && npm install {{package}}
    docker-compose exec web_dev npm install {{package}}

web-install-dev package:
    cd apps/web && npm install --save-dev {{package}}
    docker-compose exec web_dev npm install --save-dev {{package}}