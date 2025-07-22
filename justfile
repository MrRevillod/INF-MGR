run DOCKERARGS="":
    docker compose up {{DOCKERARGS}}

lint:
    cargo clippy --all-features -- -D warnings && \
    cd apps/client && npm run lint && cd ../..

fmt:
    cargo fmt --all -- --check && \
    cd apps/client && npm run format && cd ../..

db-seed:
    docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder

install:
    cd apps/client && npm install && cd ../..

web-install package="":
    cd apps/client && npm install {{package}}
    docker exec inf_mgr_client_dev npm install {{package}}

web-install-dev package:
    cd apps/client && npm install --save-dev {{package}}
    docker exec inf_mgr_client_dev npm install --save-dev {{package}}

test DOCKERARGS="":
    docker compose -f docker-compose.test.yml up {{DOCKERARGS}}