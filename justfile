
PROJECT_NAME := "INF_MGR"
TEST_ARGS := "-- --nocapture --test-threads=1"
COMPOSE_TEST_FILE := "docker-compose.test.yml"

run DOCKERARGS="":
    docker compose up {{DOCKERARGS}}

lint:
    cargo clippy --all-features -- -D warnings
    # cd apps/client && npm run lint && cd ../..

fmt:
    cargo fmt
    # cd apps/client && npm run format && cd ../..

fmt-check:
    cargo fmt --check
    # cd apps/client && npm run format && cd ../..

db-seed:
    docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder

web-install package="" mode="":
    #!/usr/bin/env bash
    if [ "{{mode}}" = "dev" ]; then
        cd apps/client && npm install --save-dev {{package}} && cd ../..
        docker exec inf_mgr_client_dev npm install --save-dev {{package}}
    else
        cd apps/client && npm install {{package}} && cd ../..
        docker exec inf_mgr_client_dev npm install {{package}}
    fi

#

test entity="" mode="":
    #!/usr/bin/env bash
    if [ "{{mode}}" = "watch" ]; then
        docker compose -f {{COMPOSE_TEST_FILE}} run --rm backend_test cargo watch -x 'test {{entity}} {{TEST_ARGS}}' -w src
    else
        docker compose -f {{COMPOSE_TEST_FILE}} run --rm backend_test cargo test {{entity}} {{TEST_ARGS}}
    fi

#

test-all mode="":
    #!/usr/bin/env bash
    if [ "{{mode}}" = "watch" ]; then
        docker compose -f {{COMPOSE_TEST_FILE}} up
    else
        docker compose -f {{COMPOSE_TEST_FILE}} run --rm backend_test cargo test {{TEST_ARGS}}
    fi