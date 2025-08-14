
PROJECT_NAME := "INF_MGR"
TEST_ARGS := "-- --nocapture --test-threads=1"
COMPOSE_TEST_FILE := "docker-compose.test.yml"

run DOCKERARGS="":
	docker compose up {{DOCKERARGS}}

lint:
	cargo clippy --all-features -- -D warnings && \
	cd apps/client && npm run lint && cd ../..

fmt:
	cargo fmt --verbose && \
	cd apps/client && npm run format && cd ../..

fmt-check:
	cargo fmt --check && \
	cd apps/client && npm run format && cd ../..

db-seed:
	docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder

web-install package="":
	cd apps/client && npm install {{package}} && cd ../..
	docker exec inf_mgr_client_dev npm install {{package}}

web-install-dev package:
	cd apps/client && npm install --save-dev {{package}} && cd ../..
	docker exec inf_mgr_client_dev npm install --save-dev {{package}}

# Testing commands

_ensure-test-service:
	@docker compose -f {{COMPOSE_TEST_FILE}} up -d backend_test postgres_test

test entity="":
    just _ensure-test-service
    docker compose -f {{COMPOSE_TEST_FILE}} exec backend_test sh -c "cd tests && cargo test {{entity}} {{TEST_ARGS}}"

test-watch entity="":
	just _ensure-test-service
	docker compose -f {{COMPOSE_TEST_FILE}} exec backend_test sh -c "cd tests && cargo watch -x test {{entity}} {{TEST_ARGS}} -w src"

test-clean:
	docker compose -f {{COMPOSE_TEST_FILE}} down -v
	docker volume rm inf-mgr_rust_target_cache inf-mgr_cargo_cache 2>/dev/null || true
	rm -f tests/config tests/tools apps/server/tools/tools apps/server/config/config