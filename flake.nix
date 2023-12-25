# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#    http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
{
  description = "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixpkgs-unstable;
    flake-utils.url = github:numtide/flake-utils;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };

    crane = {
      url = github:ipetkov/crane;
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };

    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [(import rust-overlay)];
        config.allowUnfree = true; # sorry stallman senpai :(
      };

      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      stdenv =
        if pkgs.stdenv.isLinux
        then pkgs.stdenv
        else pkgs.clangStdenv;

      terraform = pkgs.terraform.withPlugins (plugins:
        with plugins; [
          kubernetes
          helm
        ]);

      rustflags =
        if pkgs.stdenv.isLinux
        then ''-C link-arg=-fuse-ld=mold -C target-cpu=native $RUSTFLAGS''
        else ''$RUSTFLAGS'';

      craneLib = crane.lib.${system};
      commonCraneArgs = {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        buildInputs = with pkgs; [openssl];
        nativeBuildInputs = with pkgs; [pkg-config];
      };

      commonRustPlatformArgs = {
        version = "0.1.0-beta";
        src = ./.;
        cargoBuildFlags = "-C lto=true -C opt-level=s -C strip=symbols";
        cargoLock = {lockFile = ./Cargo.lock;};
      };

      dependencies = craneLib.buildDepsOnly (commonCraneArgs
        // {
          pname = "charted-deps";
        });

      clippy = craneLib.cargoClippy (commonCraneArgs
        // {
          inherit dependencies;

          pname = "charted-clippy";
        });

      charted-cli = pkgs.rustPlatform.buildRustPackage (commonRustPlatformArgs
        // {
          pname = "charted";

          nativeBuildInputs = with pkgs; [pkg-config];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        });

      charted-helm-plugin = pkgs.rustPlatform.buildRustPackage (commonRustPlatformArgs
        // {
          pname = "charted-helm-plugin";

          nativeBuildInputs = with pkgs; [pkg-config];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        });
    in rec {
      packages = {
        charted = charted-cli;
        helm-plugin = charted-helm-plugin;
        all = pkgs.symlinkJoin {
          name = "charted";
          paths = [charted-cli charted-helm-plugin];
        };

        default = packages.all;
      };

      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl]);
        nativeBuildInputs = with pkgs;
          [pkg-config]
          ++ (lib.optional stdenv.isLinux [mold])
          ++ (lib.optional stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreFoundation]);

        buildInputs = with pkgs; [
          cargo-expand
          terraform
          openssl
          glibc
          rust
          git
          bun
        ];

        shellHook = ''
          export RUSTFLAGS="--cfg tokio_unstable ${rustflags}";
        '';
      };
    });
}
