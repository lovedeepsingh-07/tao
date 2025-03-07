{
  description = "tao";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/48913d8f9127ea6530a2a2f1bd4daa1b8685d8a3";
	  rust_1_84_0-pkgs.url = "github:nixos/nixpkgs/d98abf5cf5914e5e4e9d57205e3af55ca90ffc1d";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust-pkgs = inputs.rust_1_84_0-pkgs.legacyPackages.${system};
      in {
        formatter =
          pkgs.nixfmt-classic;
        devShell = pkgs.mkShell {
          packages = [
            rust-pkgs.rustc
            rust-pkgs.cargo
            rust-pkgs.rustfmt
			rust-pkgs.just
          ];
        };
      });
}
