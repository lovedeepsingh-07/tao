export PUBLIC_APP_ENV := "development"

default:
	@just -l

run $PUBLIC_APP_RUN_METHOD="embedded":
	doppler run -- cargo run -p tao

run_tui:
	cargo run -p tao_tui

[working-directory: "./frontend"]
run_frontend $PUBLIC_APP_RUN_METHOD="api" $PUBLIC_API_URL="http://localhost:6969":
		yarn run dev

[working-directory: "./frontend"]
build_frontend $PUBLIC_APP_RUN_METHOD="embedded":
	yarn run build

test:
	cargo test

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args

fmt:
	@alejandra .
	@cargo fmt
