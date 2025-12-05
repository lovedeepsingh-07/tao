build:
	@mkdir -p build
	@rustc ./_build.rs -o build/tao_build
	@./build/tao_build

run: build
	@./build/basement
runw: build
	@wine ./build/basement

fmt:
	rustfmt ./_build.rs ./.build/tao.rs
