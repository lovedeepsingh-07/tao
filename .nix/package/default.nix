{
  pkgs,
  crane_lib,
  gitignore,
  ...
}: let
  package_name = "tao";
  src = gitignore.lib.gitignoreSource ../../.;
  workspace_toml = builtins.fromTOML (builtins.readFile (src + "/Cargo.toml"));
  package_version = workspace_toml.workspace.package.version;
  native_build_inputs = [
    pkgs.pkg-config
    pkgs.lld
    pkgs.clang
  ];
  build_inputs = [];
  common_args = {
    pname = package_name;
	inherit src;
    # src = crane_lib.cleanCargoSource src;
    version = package_version;
    strictDeps = true;
    doCheck = false;
    # below arguments are set to prevent "cargo check" command from running
    cargoCheckCommand = "";
    cargoExtraArgs = "";
    nativeBuildInputs = native_build_inputs;
    buildInputs = build_inputs;
  };
  cargo_artifacts = crane_lib.buildDepsOnly common_args;
in rec {
  frontend = pkgs.callPackage ./frontend.nix {
    inherit gitignore;
    yarn_berry = pkgs.yarn-berry_4;
    frontend_src = ../../frontend;
  };
  default = crane_lib.mkCargoDerivation (common_args
    // {
      pname = package_name;
      cargoArtifacts = cargo_artifacts;
      preBuild = ''
         	mkdir -p frontend/build
        cp -r ${frontend}/dist/* frontend/build/
      '';
      buildPhaseCargoCommand = "cargo build -p ${package_name} --release";
      installPhase = ''
         	mkdir -p $out/bin
        cp -r target/release/${package_name} $out/bin/
      '';
    });
}
