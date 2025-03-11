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
  lib,
  rustPlatform,
  fetchFromGitLab,
}:
rustPlatform.buildRustPackage rec {
  pname = "cargo-upgrades";
  version = "2.2.0";

  src = fetchFromGitLab {
    owner = "kornelski";
    repo = "cargo-upgrades";
    rev = "v${version}";
    hash = "sha256-b86ghds8hWllMmPa7cqfiW6sq9Pv9bKL2DLaJVz1Sww=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-yEUfWe4/kSvBPx3xneff45+K3Gix2QXDjUesm+psUxI=";

  meta = with lib; {
    description = "Checks if dependencies in Cargo.toml are up to date. Compatible with workspaces and path dependencies.";
    mainProgram = "cargo-upgrades";
    homepage = "https://github.com/kornelski/cargo-upgrades";
    licenses = with licenses; [gpl3Plus];
    maintainers = with maintainers; [auguwu];
  };
}
