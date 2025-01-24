{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import inputs.nixpkgs {
          inherit system;
        };
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          version = "1.0.0";
          pname = "artifact-loader";
          cargoHash = "sha256-9qniTJks3QdiLGNNAuDqX9v/knm3GYNavH4y7YEH8ZU=";
          src = ./.;
        };
      }
    );
}
