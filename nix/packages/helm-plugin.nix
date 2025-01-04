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
  makeRustPlatform,
  pkg-config,
  installShellFiles,
  openssl,
  sqlite,
  postgresql,
  darwin,
  stdenv,
  rust-bin,
  lib,
}: let
  common = import ../common.nix;
  rust-toolchain = common.mkRustPlatform rust-bin;
  rustPlatform = common.mkNixpkgsRustPlatform {inherit makeRustPlatform;} rust-toolchain;
  version = common.cargoTOML.workspace.package.version;
in
  rustPlatform.buildRustPackage {
    inherit version;

    pname = "charted-helm-plugin";
    src = ../../.;

    cargoBuildFlags = ["--package" "charted-helm-plugin"];
    cargoLock = {
      inherit (common) outputHashes;

      lockFile = ../../Cargo.lock;
    };

    checkFlags = [
      # TODO(@auguwu/@spotlightishere): try to fix this on Nix. Do we need special logic for this?
      # It works in CI and on our machines, so we aren't sure why it is failing.
      "--skip=ulid::tests::test_monotonicity"
    ];

    nativeBuildInputs = [pkg-config installShellFiles];
    buildInputs =
      [openssl sqlite postgresql]
      ++ (lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
        CoreFoundation
        Security
        SystemConfiguration
      ]));

    env.CHARTED_DISTRIBUTION_KIND = "nix";

    postPatch = ''
      sed -i '/^hooks:/,+2 d' plugin.yaml
    '';

    postInstall = ''
      install -Dm644 plugin.yaml $out/charted-helm-plugin/plugin.yaml
      mv $out/bin $out/charted-helm-plugin
    '';

    meta = with lib; {
      description = "🐻‍❄️📦 Free, open-source way to distribute Helm charts across the world";
      maintainers = with maintainers; [auguwu spotlightishere noelware];
      mainProgram = "charted";
      changelog = "https://charts.noelware.org/changelogs/charted#v${version}";
      homepage = "https://charts.noelware.org";
      license = with licenses; [asl20];
    };
  }
