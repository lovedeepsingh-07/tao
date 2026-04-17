{
  pkgs,
  rust_pkg,
  ...
}: let
  rust_platform = pkgs.makeRustPlatform {
    cargo = rust_pkg;
    rustc = rust_pkg;
  };
in {
  default = rust_platform.buildRustPackage {
    pname = "tao";
    version = "0.1.0";
    src = ../.;
    buildInputs = [];
    nativeBuildInputs = [pkgs.pkg-config];
    cargoLock.lockFile = ../Cargo.lock;
  };
}
