{
  description = "tao";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/3146c6aa9995e7351a398e17470e15305e6e18ff";
    rust_overlay = {
      url = "github:oxalica/rust-overlay/59e4ab96304585fde3890025fd59bd2717985cc1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
  };
  outputs = {...} @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [(import inputs.rust_overlay)];
          config = {
            allowUnfree = true;
          };
        };
        rust_pkg = pkgs.rust-bin.stable."1.88.0".default;
      in {
        devShells = import ./.nix/shell.nix {inherit pkgs rust_pkg;};
        packages = import ./.nix/pkg.nix {inherit pkgs rust_pkg;};
      }
    );
}
