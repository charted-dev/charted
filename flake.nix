# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

      rustflags =
        if pkgs.stdenv.isLinux
        then ''-C link-arg=-fuse-ld=mold -C target-cpu=native $RUSTFLAGS''
        else ''$RUSTFLAGS'';
    in rec {
      packages = {
        charted = pkgs.rustPlatform.buildRustPackage {
          nativeBuildInputs = with pkgs; [pkg-config protobuf];
          buildInputs = with pkgs; [openssl];
          cargoSha256 = pkgs.lib.fakeSha256;
          version = "0.1.0-beta";
          name = "charted";
          src = ./.;

          env.PROTOC = pkgs.lib.getExe pkgs.protobuf;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "noelware-config-0.1.0" = "sha256-Y6Yf3TU0vzhU1UVdIdrDaECD5tvDwQZ9CYyDXwxmpe8=";
              "noelware-config-derive-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-log-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-remi-0.1.0" = pkgs.lib.fakeSha256;
              "noelware-serde-0.1.0" = pkgs.lib.fakeSha256;
            };
          };

          meta = with pkgs.lib; {
            description = "Free, open source, and reliable Helm chart registry in Rust";
            homepage = "https://charts.noelware.org";
            license = with licenses; [asl20];
            maintainers = with maintainers; [auguwu spotlightishere];
            mainProgram = "charted";
          };
        };

        helm-plugin = import ./nix/helm-plugin;
        default = packages.charted;
      };

      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl]);
        nativeBuildInputs = with pkgs;
          [pkg-config]
          ++ (lib.optional stdenv.isLinux [mold lldb gdb])
          ++ (lib.optional stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreFoundation]);

        buildInputs = with pkgs; [
          cargo-expand
          sqlx-cli
          sccache
          openssl
          glibc
          rust
          git
        ];

        shellHook = ''
          export RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"
          export RUSTFLAGS="--cfg tokio_unstable ${rustflags}"
        '';
      };
    });
}
