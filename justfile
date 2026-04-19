export PUBLIC_APP_ENV := "development"
export PUBLIC_API_URL := "http://localhost:6969"

run:
	PUBLIC_APP_RUN_METHOD="embedded" \
		cargo run -p tao

[working-directory: "./frontend"]
run_frontend:
	PUBLIC_APP_RUN_METHOD="api" \
		bun run dev

[working-directory: "./frontend"]
build_frontend:
	PUBLIC_APP_RUN_METHOD="api" \
	bun run build


test:
	cargo test

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt

