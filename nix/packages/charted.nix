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

    pname = "charted";
    src = ../../.;

    cargoBuildFlags = ["--bin" "charted"];
    useFetchCargoVendor = true;
    cargoHash = "sha256-bvhzN8qjR50vEJ4nES1m9a5W/QvsEjIz1L1aKKIB/oA=";

    nativeBuildInputs = [pkg-config installShellFiles];
    buildInputs =
      [openssl sqlite postgresql]
      ++ (lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
        CoreFoundation
        Security
        SystemConfiguration
      ]));

    env.CHARTED_DISTRIBUTION_KIND = "nix";

    postInstall = ''
      installShellCompletion --cmd charted \
        --bash <($out/bin/charted completions bash) \
        --fish <($out/bin/charted completions fish) \
        --zsh  <($out/bin/charted completions zsh)
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
