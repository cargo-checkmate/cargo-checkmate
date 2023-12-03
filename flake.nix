# A nix flake for `cargo-checkmate`.
#
# Nix users can start a shell w/ checkmate available with:
#
# ```
# $ nix shell 'github:cargo-checkmate/cargo-checkmate'
# ```
#
# -or they can install it for their user profile with:
#
# ```
# $ nix profile install 'github:cargo-checkmate/cargo-checkmate'
# ```
#
# Note: this nix flake includes all transitive runtime dependencies,
# such as cargo and gcc to work "out of the box".
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [
        nci.flakeModule
        ./crates.nix
      ];
      perSystem = {
        pkgs,
        config,
        ...
      }: let
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs."cargo-checkmate";
        cargo-checkmate-pkg = crateOutputs.packages.release;

        # We need the rust toolchain as a runtime dependency:
        wrapped-pkg = pkgs.symlinkJoin {
          name = "${cargo-checkmate-pkg.name}-with-bundled-cargo";
          paths = [
            pkgs.gcc
            pkgs.cargo
            pkgs.clippy
            pkgs.rustfmt
            cargo-checkmate-pkg
          ];
        };
      in {
        # export the crate devshell as the default devshell
        devShells.default = crateOutputs.devShell;
        packages.default = wrapped-pkg;
        # Expose the unwrapped package if some consumers need it:
        packages.unwrapped = cargo-checkmate-pkg;
      };
    };
}
