export PUBLIC_APP_ENV := "development"

run:
	cargo run -p tao

[working-directory: "./frontend"]
run_frontend:
	bun run dev

[working-directory: "./frontend"]
build_frontend:
	bun run build


test:
	cargo test

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args
