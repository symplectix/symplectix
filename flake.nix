# Setup a reproducible build/test environment using a [flake].
#
# - https://devenv.sh/
# - https://github.com/tweag/rules_nixpkgs
#
# [flake]: https://nixos.wiki/wiki/Flakes
{
  description = "trunk";

  nixConfig = {
    bash-prompt = "\[nix\]$ ";
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
          };
        in
        {
          devShells.default = pkgs.mkShellNoCC {
            buildInputs = with pkgs;[
              (writeShellScriptBin "bazel" ''
                exec ${bazelisk}/bin/bazelisk "$@"
              '')
              bazel-buildtools
            ];
          };
        }
      );
}
