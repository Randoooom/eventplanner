SURREALDB_VERSION ?= v2.1.4

clippy:
	cargo clippy -- -D warnings

check:
	cargo check

surrealdb:
	docker run --name surrealdb -d -p 8080:8000 surrealdb/surrealdb:${SURREALDB_VERSION} start -u root -p root --allow-scripting && sleep 1 

stop-surrealdb:
	docker rm -f surrealdb || true

test: stop-surrealdb surrealdb
	SURREALDB_USERNAME=root SURREALDB_PASSWORD=root SURREALDB_ENDPOINT=localhost:8080 cargo test -- --nocapture --test-threads=1

	${MAKE} stop-surrealdb

run: stop-surrealdb surrealdb
	SURREALDB_USERNAME=root SURREALDB_PASSWORD=root SURREALDB_ENDPOINT=localhost:8080 RUST_LOG=INFO cargo run

dev: run 
	cd frontend && pnpm run dev
