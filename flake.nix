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
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [(import rust-overlay)];
        config.allowUnfree = true; # im so sorry stallman senpai :(
      };

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

      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      bazel = pkgs.bazel_6;
    in {
      devShells.default = pkgs.mkShell {
        # TODO(@auguwu): uncomment once we are in a release of `rules_rust`, not
        #                in a commit.
        #
        # NIX_LD = "${stdenv.cc}/nix-support/dynamic-linker";
        # NIX_LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
        #   stdenv.cc.cc
        #   openssl
        #   curl
        # ]);

        nativeBuildInputs = with pkgs;
          [pkg-config git]
          ++ (lib.optional stdenv.isLinux [mold lldb])
          ++ (lib.optional stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreFoundation]);

        buildInputs = with pkgs; [
          nodePackages.pnpm

          cargo-expand
          terraform
          nodejs_20
          clang_16
          openssl
          bazel_6
          cargo
          llvm
          mold
          rust
          git
        ];

        shellHook = ''
          export RUSTFLAGS="--cfg tokio_unstable ${rustflags}"
        '';
      };
    });
}
