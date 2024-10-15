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
{pkgs}:
with pkgs; let
  common = import ./common.nix;
  toolchain = common.mkRustPlatform pkgs.rust-bin;
  rustflags = common.rustflags stdenv;
in
  mkShell {
    LD_LIBRARY_PATH = lib.makeLibraryPath [openssl sqlite postgresql];
    nativeBuildInputs =
      [pkg-config sqlite postgresql.lib]
      ++ (lib.optional stdenv.isLinux [mold lldb])
      ++ (lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
        CoreFoundation
        Security
        SystemConfiguration
      ]));

    buildInputs =
      [
        cargo-nextest # replacement for `cargo test`
        cargo-machete # used to validate dependencies
        cargo-deny # used to validate licenses, vulns, etc.

        # I don't plan to add MySQL support and probably never will.
        (diesel-cli.override {
          mysqlSupport = false;
          postgresqlSupport = true;
          sqliteSupport = true;
        })

        toolchain # rust toolchain
        hadolint
        openssl
        git
      ]
      ++ (lib.optional stdenv.isLinux [glibc]);

    shellHook = ''
      export RUSTFLAGS="--cfg tokio_unstable ${rustflags}"
    '';
  }
