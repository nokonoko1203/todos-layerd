build:
	cargo build --release

run:
	docker compose up -d --build

stop:
	docker compose down

test:
	cargo test

dev:
	cargo run
