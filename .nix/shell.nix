{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    nativeBuildInputs = [
      pkgs.alejandra
      rust_pkg
      pkgs.bun
      pkgs.pkg-config
      pkgs.jq
      pkgs.lld
      pkgs.clang
    ];
    buildInputs = [
    ];
    shellHook = ''zsh'';
  };
}
