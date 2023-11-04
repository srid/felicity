{
  description = "A starter template for Dioxus Desktop apps w/ Tailwind & Nix";
  nixConfig = {
    # https://garnix.io/docs/caching
    extra-substituters = "https://cache.garnix.io";
    extra-trusted-public-keys = "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=";
  };
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";

    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    cargo-doc-live.url = "github:srid/cargo-doc-live";

    dioxus-desktop-template.url = "github:srid/dioxus-desktop-template";
    dioxus-desktop-template.flake = false;
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.process-compose-flake.flakeModule
        inputs.cargo-doc-live.flakeModule
        inputs.pre-commit-hooks-nix.flakeModule
        (inputs.dioxus-desktop-template + /nix/flake-module.nix)
      ];

      flake = {
        nix-health.default = {
          nix-version.min-required = "2.16.0";
          direnv.required = true;
        };
      };

      perSystem = { config, self', pkgs, lib, system, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.rust-overlay.overlays.default
          ];
        };

        # Add your auto-formatters here.
        # cf. https://numtide.github.io/treefmt/
        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        pre-commit = {
          check.enable = true;
          settings.hooks = {
            nil.enable = true;
          };
        };

        dioxus-desktop = {
          src = lib.cleanSourceWith {
            src = ./.; # The original, unfiltered source
            filter = path: type:
              (lib.hasSuffix "\.html" path) ||
              (lib.hasSuffix "tailwind.config.js" path) ||
              # Example of a folder for images, icons, etc
              (lib.hasInfix "/assets/" path) ||
              (lib.hasInfix "/css/" path) ||
              (lib.hasInfix "/.sqlx/" path) ||
              # Default filter from crane (allow .rs files)
              (config.dioxus-desktop.craneLib.filterCargoSources path type)
            ;
          };

          rustBuildInputs = (with pkgs; [
          ] ++ lib.optionals pkgs.stdenv.isLinux
            [
              alsa-lib
              webkitgtk_4_1
            ] ++ lib.optionals pkgs.stdenv.isDarwin (
            with darwin.apple_sdk.frameworks; [
              IOKit
              Carbon
              WebKit
              Security
              Cocoa
            ]
          ));
        };

        packages.default = self'.packages.felicity;

        devShells.default = pkgs.mkShell {
          name = "nix-browser";
          inputsFrom = [
            config.treefmt.build.devShell
            config.pre-commit.devShell
            self'.devShells.felicity
          ];
          packages = with pkgs; [
            just
            nixci

            # App deps
            dioxus-cli
            sqlx-cli
          ];
          shellHook = ''
            echo
            echo "🍎🍎 Run 'just <recipe>' to get started"
            just
          '';
        };
      };
    };
}
