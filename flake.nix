{
  description = "leafyPuff sync API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.leafypuff-api = pkgs.rustPlatform.buildRustPackage {
          pname = "leafypuff-api";
          version = "0.16.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # The workspace also holds the Android-facing core crate; only the API is deployable.
          cargoBuildFlags = [ "-p" "leafypuff-api" ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          doCheck = false;
        };

        packages.default = self.packages.${system}.leafypuff-api;
      }
    )
    // {
      nixosModules.default = import ./nix/module.nix self;
    };
}
