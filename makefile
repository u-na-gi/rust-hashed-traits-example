build:
	docker compose up -d --build

test:
	make build
	docker compose exec rust-hashed-traits-example cargo test