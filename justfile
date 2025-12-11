build example:
	@mkdir -p build/examples/{{example}}
	@rustc tao.rs --crate-name=tao --crate-type=lib -o build/libtao.rlib
	@rustc examples/{{example}}/_build.rs -Lbuild -o build/examples/{{example}}/tao_build

run example: (build example)
	@cd examples/{{example}} && ../../build/examples/{{example}}/tao_build
	@ninja -C build/examples/{{example}} -f build.ninja

fmt:
	rustfmt ./*.rs
