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
  description = "charted is a free, open, and reliable way to distribute Helm charts";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs = {
        follows = "nixpkgs";
      };
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      rustflags =
        if pkgs.stdenv.isLinux
        then ''-C link-arg=-fuse-ld=mold -C target-cpu=native $RUSTFLAGS''
        else ''$RUSTFLAGS'';

      # We use Rust nightly for all crates, so we want to match it.
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      charted = rustPlatform.buildRustPackage {
        nativeBuildInputs = with pkgs; [pkg-config installShellFiles];
        buildInputs = with pkgs; [openssl sqlite postgresql];
        version = "${cargoTOML.workspace.package.version}";
        name = "charted";
        src = ./.;

        env.CHARTED_DISTRIBUTION_KIND = "nix";
        cargoLock = {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "azalia-0.1.0" = "sha256-ftI7yUzqhjoTk8dl/4+zkXYai1rG6PF3t5anhOElgLM=";
          };
        };

        postInstall = ''
          installShellCompletion --cmd charted \
            --bash <($out/bin/charted completions bash) \
            --fish <($out/bin/charted completions fish) \
            --zsh <($out/bin/charted completions zsh)
        '';

        useNextest = true;
        meta = with pkgs.lib; {
          description = "charted is a free, open, and reliable way to distribute Helm charts";
          homepage = "https://charts.noelware.org";
          license = with licenses; [asl20];
          maintainers = with maintainers; [auguwu spotlightishere noelware];
          mainProgram = "charted";
        };
      };

      helm-plugin = rustPlatform.buildRustPackage {
        nativeBuildInputs = with pkgs; [pkg-config protobuf installShellFiles];
        buildInputs = with pkgs; [openssl];
        cargoSha256 = pkgs.lib.fakeSha256;
        version = "${cargoTOML.workspace.package.version}";
        name = "charted-helm-plugin";
        src = ./.;

        cargoBuildFlags = ["--package" "charted-helm-plugin"];
        cargoLock = {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "azalia-0.1.0" = "sha256-ftI7yUzqhjoTk8dl/4+zkXYai1rG6PF3t5anhOElgLM=";
          };
        };

        postPatch = ''
          sed -i '/^hooks:/,+2 d' plugin.yaml
        '';

        postInstall = ''
          installShellCompletion --cmd charted-helm-plugin \
            --bash <($out/bin/charted-helm-plugin completions bash) \
            --fish <($out/bin/charted-helm-plugin completions fish) \
            --zsh <($out/bin/charted-helm-plugin completions zsh)

          install -Dm755 $out/charted-helm-plugin
          install -Dm644 plugin.yaml $out/charted-helm-plugin/plugin.yaml
          mv $out/bin $out/charted-helm-plugin
        '';

        useNextest = true;
        meta = with pkgs.lib; {
          description = "Helm plugin to help aid developing Helm charts with charted";
          homepage = "https://charts.noelware.org";
          license = with licenses; [asl20];
          maintainers = with maintainers; [auguwu spotlightishere noelware];
        };
      };
    in {
      packages = {
        inherit charted helm-plugin;

        default = charted;
        all = pkgs.symlinkJoin {
          name = "charted-${cargoTOML.workspace.package.version}";
          paths = [charted helm-plugin];
        };
      };

      devShells.default = pkgs.mkShell {
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [openssl sqlite postgresql]);
        nativeBuildInputs = with pkgs;
          [pkg-config sqlite postgresql.lib]
          ++ (lib.optional stdenv.isLinux [mold lldb])
          ++ (lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [CoreFoundation Security]));

        buildInputs = with pkgs; [
          cargo-nextest
          cargo-machete
          cargo-deny

          # I don't plan to add MySQL support and probably never will.
          (diesel-cli.override {
            mysqlSupport = false;
            postgresqlSupport = true;
            sqliteSupport = true;
          })

          openssl
          git

          rust
        ]
        ++ (lib.optional stdenv.isLinux [glibc]);

        shellHook = ''
          export RUSTFLAGS="--cfg tokio_unstable ${rustflags}"
        '';
      };
    });
}
