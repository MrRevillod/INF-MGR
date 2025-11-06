PROJECT_NAME := "INF_MGR"

run DOCKERARGS="":
	docker compose up {{DOCKERARGS}}

db:
	pgcli postgres://user:password@localhost:5433/inf_mgr_db

lint:
	cargo clippy --all-features -- -D warnings && \
	cd apps/client && npm run lint && cd ../.. 

fmt:
	cargo fmt --verbose && \
	cd apps/client && npm run format && cd ../..

fmt-check:
	cargo fmt --check && \
	cd apps/client && npm run format && cd ../..

check:
	cargo check --all-features
	cd apps/client && npm run check && cd ../..

db-seed:
	docker exec inf_mgr_server_dev cargo run -p server --bin seeder --features seeder

web-install package="":
	cd apps/client && npm install {{package}} && cd ../..
	docker exec inf_mgr_client_dev npm install {{package}}

web-install-dev package:
	cd apps/client && npm install --save-dev {{package}} && cd ../..
	docker exec inf_mgr_client_dev npm install --save-dev {{package}}
