{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    packages = [
      pkgs.alejandra
      rust_pkg
      pkgs.yarn-berry_4
      pkgs.yarn-berry_4.yarn-berry-fetcher
      pkgs.pkg-config
      pkgs.jq
      pkgs.lld
      pkgs.clang
      pkgs.sqlx-cli
    ];
    shellHook = ''zsh'';
  };
}
