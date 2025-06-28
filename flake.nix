{
  description = "A minimal flake for the multiversal-rss-reader Rust project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # Import project metadata from Cargo.toml
        manifest = pkgs.lib.importTOML ./Cargo.toml;
      in {
        # Package output using buildRustPackage
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          version = manifest.package.version;
          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        # Development shell with common Rust tools
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc cargo rustfmt clippy rust-analyzer
          ];
        };
      }
    );
}