BINARY_NAME := "basement"
TOOLCHAIN_FILE := env_var('PWD') + "/mingw-w64-toolchain.cmake"

default:
	@just -l

setup:
	@nix run .#setup

build-fmt:
	@mkdir -p build/fmt/install
	@cmake -S deps/fmt -B build/fmt \
		-DCMAKE_TOOLCHAIN_FILE={{TOOLCHAIN_FILE}} \
		-DFMT_TEST=OFF \
		-DCMAKE_INSTALL_PREFIX=build/fmt/install
	@cmake --build ./build/fmt
	@cmake --install ./build/fmt

build-raylib:
	@mkdir -p build/raylib/install
	@cmake -S deps/raylib -B build/raylib \
		-DCMAKE_TOOLCHAIN_FILE={{TOOLCHAIN_FILE}} \
		-DBUILD_EXAMPLES=OFF \
		-DCMAKE_INSTALL_PREFIX=build/raylib/install
	@cmake --build ./build/raylib
	@cmake --install ./build/raylib

build: build-fmt build-raylib
	@x86_64-w64-mingw32-g++ ./src/main.cpp  \
		-Ibuild/raylib/install/include -Lbuild/raylib/install/lib -lraylib \
		-lgdi32 -lwinmm \
		-Ibuild/fmt/install/include -Lbuild/fmt/install/lib -lfmt \
		-o build/{{BINARY_NAME}}

run: build
	@./build/basement.exe
