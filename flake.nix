{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crane }:
    utils.lib.eachSystem ["aarch64-linux" "x86_64-linux"] (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs { inherit system overlays; };
        craneLib = crane.mkLib pkgs;

        # Build inputs
        requiredPrograms = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
        ];

        # Compiletime/Runtime deps (native linux build)
        requiredLibsLinux = with pkgs; [];

        # IDE/shell dependencies
        developPrograms = with pkgs; [
          clippy
          rust-analyzer-unwrapped
          cargo-edit
        ];

        # Autofetch project info from Cargo
        cargoDesc = pkgs.lib.trivial.importTOML ./crates/game/Cargo.toml;
        projectName = cargoDesc.package.name;
        projectVersion = cargoDesc.package.version;

        packageDef = rec {
          pname = projectName;
          version = projectVersion;

          src =
            pkgs.lib.cleanSourceWith {
              src = ./.;
              filter = path: type: craneLib.filterCargoSources path type;
            };

          # https://github.com/NixOS/nix/issues/4623
          # GIT_LFS_SKIP_SMUDGE = 1;
          strictDeps = true;
          nativeBuildInputs = requiredPrograms;
        };
      in
      rec {
        # `nix develop`
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = developPrograms ++ requiredPrograms;
          buildInputs = requiredLibsLinux;
        };
      });
}
