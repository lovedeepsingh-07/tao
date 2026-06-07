{
  stdenv,
  frontend_src,
  gitignore,
  yarn_berry,
}: let
  src = gitignore.lib.gitignoreSource frontend_src;
  package_json = builtins.fromJSON (builtins.readFile "${src}/package.json");
  missing_hashes = "${src}/missing-hashes.json";
in
  stdenv.mkDerivation rec {
    pname = package_json.name;
    version = package_json.version;
    inherit src;
    nativeBuildInputs = [
      yarn_berry.yarnBerryConfigHook
      yarn_berry
    ];
    missingHashes = missing_hashes;
    offlineCache = yarn_berry.fetchYarnBerryDeps {
      inherit src;
      missingHashes = missing_hashes;
      hash = "sha256-0HTxnzu6DNXTY9pcCPbf4DYM1aQYx1TR31KudoPUd0A=";
    };
    PUBLIC_APP_RUN_METHOD = "embedded"; # builds the frontend for embedding in the backend as an SPA
    buildPhase = ''
      yarn run build
    '';
    installPhase = ''
         mkdir -p $out/dist/
         cp -r build/* $out/dist/
    '';
  }
