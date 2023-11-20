run-api:
	cd api && cargo-watch -x run

run-cli:
	cd cli && cargo run -- $(ARGS)
