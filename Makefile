SURREALDB_VERSION ?= v2.1.4

clippy:
	cargo clippy

check:
	cargo check

surrealdb:
	docker run --name surrealdb -d -p 8080:8000 surrealdb/surrealdb:${SURREALDB_VERSION} start -u root -p root && sleep 1 

stop-surrealdb:
	docker rm -f surrealdb || true

test: stop-surrealdb surrealdb
	SURREALDB_USERNAME=root SURREALDB_PASSWORD=root SURREALDB_ENDPOINT=localhost:8080 cargo test -- --nocapture --test-threads=1

	${MAKE} stop-surrealdb
