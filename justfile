export PUBLIC_APP_ENV := "development"

run:
	cargo run -p tao

test:
	cargo test

lint:
	@cargo clippy -- \
		--allow clippy::needless_return \
		--allow clippy::uninlined_format_args
