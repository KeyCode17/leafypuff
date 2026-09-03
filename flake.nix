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
          version = "0.39.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # The workspace also holds the Android-facing core crate; only the API is deployable.
          cargoBuildFlags = [ "-p" "leafypuff-api" ];
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
          doCheck = false;
        };

        # The CMS is a static bundle. It is built here rather than on the host so the deploy
        # carries a fixed artifact, and pnpmDeps pins the dependency closure by hash the same way
        # cargoLock pins the rust one.
        packages.leafypuff-web = pkgs.stdenv.mkDerivation (final: {
          pname = "leafypuff-web";
          version = "0.39.1";
          src = ./apps/web;

          nativeBuildInputs = [
            pkgs.nodejs_24
            pkgs.pnpm_10.configHook
          ];

          # pnpm 10 reads the committed lockfileVersion 9.0 unchanged. The default in this nixpkgs
          # is pnpm 11, whose fetcher wants a lockfile this repo has not moved to yet.
          pnpmDeps = pkgs.pnpm_10.fetchDeps {
            inherit (final) pname version src;
            fetcherVersion = 3;
            hash = "sha256-AWnPv617BDmy+lOLTCgWxvGBuZ/78a9Jay32Lkijjvg=";
          };

          env.VITE_API_BASE_URL = "https://leafypuff-api.daffakaryudi.web.id";

          buildPhase = ''
            runHook preBuild
            pnpm exec vite build
            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            cp -r dist $out
            runHook postInstall
          '';
        });

        packages.default = self.packages.${system}.leafypuff-api;
      }
    )
    // {
      nixosModules.default = import ./nix/module.nix self;
    };
}
