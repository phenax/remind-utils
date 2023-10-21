{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    let
      shell = { pkgs, system }:
        with pkgs;
        let
          rust = rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [
              "x86_64-unknown-linux-gnu"
            ];
          };
        in

        mkShell rec {
          buildInputs = [
            rust
            rust-analyzer-unwrapped
            just
            cargo-watch
          ];
          nativeBuildInputs = [ clang ];

          # RUST_BACKTRACE = 1;
          LIBCLANG_PATH = "${libclang.lib}/lib";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath (buildInputs ++ nativeBuildInputs)}";
        };
    in
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          devShells.default = shell { inherit pkgs system; };
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock = { lockFile = ./Cargo.lock; };
          };
        });
}
