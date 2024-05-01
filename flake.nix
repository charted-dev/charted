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

      cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      stdenv =
        if pkgs.stdenv.isLinux
        then pkgs.stdenv
        else pkgs.clangStdenv;

      rustflags =
        if pkgs.stdenv.isLinux
        then ''-C link-arg=-fuse-ld=mold -C target-cpu=native $RUSTFLAGS''
        else ''$RUSTFLAGS'';

      # create a Rust platform that uses our rust-toolchain.toml since Nix doesn't
      # detect it
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      hashes = {
        "noelware-serde" = "sha256-ZOIaeMJO44NNn2/PKiLX731UlKAQukYAlSWQixELxl4=";
        "azalia" = "sha256-jHYJRLjVmakQIJeD/Xq+AOTm4icBEoqBNJWJpTpS8KM=";
      };

      mkPackage = {
        description,
        mainProgram,
        name,
      }:
        rustPlatform.buildRustPackage {
          inherit name;

          nativeBuildInputs = with pkgs; [pkg-config protobuf installShellFiles];
          buildInputs = with pkgs; [openssl];
          cargoSha256 = pkgs.lib.fakeSha256;
          version = "${cargoTOML.workspace.package.version}";
          src = ./.;

          env.PROTOC = pkgs.lib.getExe pkgs.protobuf;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "noelware-serde-0.1.0" = hashes."noelware-serde";
              "azalia-0.1.0" = hashes.azalia;
            };
          };

          useNextest = true;
          meta = with pkgs.lib; {
            inherit description mainProgram;

            homepage = "https://charts.noelware.org";
            license = with licenses; [asl20];
            maintainers = with maintainers; [auguwu spotlightishere];
          };
        };

      charted = mkPackage {
        description = "Free, open source, and reliable Helm chart registry in Rust";
        mainProgram = "charted";
        name = "charted";
      };

      helm-plugin =
        mkPackage {
          description = "Faciliate common practices with Helm + charted-server easily as a plugin";
          mainProgram = "helm charted";
          name = "charted-helm-plugin";
        }
        // {
          cargoBuildFlags = ["--package" "charted-helm-plugin"];

          # NOTE: Remove the install and upgrade hooks.
          postPatch = ''
            sed -i '/^hooks:/,+2 d' plugin.yaml
          '';

          postInstall = ''
            installShellCompletion --cmd charted-helm-plugin \
              --bash <($out/bin/charted-helm-plugin completions bash) \
              --fish <($out/bin/charted-helm-plugin completions fish) \
              --zsh <($out/bin/charted-helm-plugin completions zsh)

            install -dm755 $out/charted-helm-plugin
            install -Dm644 plugin.yaml $out/charted-helm-plugin/plugin.yaml
            mv $out/bin $out/charted-helm-plugin
          '';
        };
    in {
      packages = {
        inherit charted helm-plugin;

        all = pkgs.symlinkJoin {
          name = "charted-${cargoTOML.workspace.package.version}";
          paths = [charted helm-plugin];
        };

        default = charted;
      };

      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl]);
        nativeBuildInputs = with pkgs;
          [pkg-config]
          ++ (lib.optional stdenv.isLinux [mold lldb gdb])
          ++ (lib.optional stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreFoundation]);

        buildInputs = with pkgs; [
          cargo-llvm-lines
          cargo-nextest
          cargo-machete
          cargo-expand
          cargo-deny
          sqlx-cli

          openssl
          glibc
          rust
          git
        ];

        shellHook = ''
          export RUSTFLAGS="--cfg tokio_unstable ${rustflags}"
        '';
      };
    });
}
