# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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
  description = "🐻‍❄️📦 Free, open-source way to distribute Helm charts across the world";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs = {
        follows = "nixpkgs";
      };
    };

    noelware = {
      url = "github:Noelware/nixpkgs-noelware";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    systems,
    noelware,
    ...
  }: let
    eachSystem = nixpkgs.lib.genAttrs (import systems);
    overlays = [
      (import rust-overlay)
      (import noelware)

      (final: prev: {
        # as of nixpkgs/nixpkgs-unstable@8bc6cf8907b5f38851c7c0a7599bfa2ccf0a29eb (14-04-2025),
        # bun is still at v1.2.8 and we need v1.2.9 for `S3Client.list`, which charted-server uses in
        # the src/ci/other/buildVersionJson.js script.
        #
        # TODO(@auguwu/@spotlightishere): remove this overlay once bun 1.2.9 is on nixpkgs-unstable
        bun = let
          inherit (prev) fetchurl stdenvNoCC;

          version = "1.2.10";
          sources = {
            aarch64-darwin = fetchurl {
              url = "https://github.com/oven-sh/bun/releases/download/bun-v${version}/bun-darwin-aarch64.zip";
              hash = "sha256-B4le8PtmEkm4awtyO2WxzEeQx/NoW2PNqQEisAKZlyw=";
            };

            aarch64-linux = fetchurl {
              url = "https://github.com/oven-sh/bun/releases/download/bun-v${version}/bun-linux-aarch64.zip";
              hash = "sha256-VFkv0CN+PskaKTPf8BXhWniYnZcjQELn1TNKTArVBgM=";
            };

            x86_64-darwin = fetchurl {
              url = "https://github.com/oven-sh/bun/releases/download/bun-v${version}/bun-darwin-x64-baseline.zip";
              hash = "sha256-wkFtHbo9P80XYa1ytpXaUPFElJbGrQNeadQkp4ZEEUQ=";
            };

            x86_64-linux = fetchurl {
              url = "https://github.com/oven-sh/bun/releases/download/bun-v${version}/bun-linux-x64.zip";
              hash = "sha256-aKFU/xvpaFG00ah8xRl/An74Crea+j1FhxUPrlw0w24=";
            };
          };
        in
          prev.bun.overrideAttrs (old: {
            inherit version;

            src = sources.${stdenvNoCC.hostPlatform.system} or (throw "unsupported system: ${stdenvNoCC.hostPlatform.system}");
          });
      })
    ];

    nixpkgsFor = system:
      import nixpkgs {
        inherit system overlays;
      };
  in {
    formatter = eachSystem (system: (nixpkgsFor system).alejandra);
    packages = eachSystem (system: let
      pkgs = nixpkgsFor system;
      charted = pkgs.callPackage ./nix/packages/charted.nix {};
    in {
      inherit charted;

      default = charted;
    });

    devShells = eachSystem (system: let
      pkgs = nixpkgsFor system;
    in {
      default = import ./nix/devshell.nix {inherit pkgs;};
    });
  };
}
