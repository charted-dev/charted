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
# ----
# adapted from tweag/rules_nixpkgs to include `cargo-expand` and to use the Rust
# toolchain declared in ./rust-toolchain.toml
#
# original template: https://github.com/tweag/rules_nixpkgs/blob/master/toolchains/rust/rust.bzl#L13-L119
let
  flakeLock = builtins.fromJSON (builtins.readFile ../flake.lock);
  pkgs = import <nixpkgs> {
    config = {};
    overrides = [];
    overlays = [
      (
        import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/${flakeLock.nodes.rust-overlay.locked.rev}.tar.gz")
      )
    ];
  };

  rust = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
  os = pkgs.rust.toTargetOs pkgs.stdenv.targetPlatform;
  build-triple = pkgs.rust.toRustTargetSpec pkgs.stdenv.buildPlatform;
  target-triple = pkgs.rust.toRustTargetSpec pkgs.stdenv.targetPlatform;
  binary-ext = "";
  staticlib-ext = ".a";
  dylib-ext =
    if os == "macos"
    then ".dylib"
    else ".so";
in
  pkgs.buildEnv {
    extraOutputsToInstall = ["out" "bin" "lib"];
    paths = with pkgs; [cargo-expand rust];
    name = "bazel-rust-toolchain";
    postBuild = ''
      cat <<EOF > $out/BUILD
      package(default_visibility = ["//visibility:public"])

      filegroup(
          name = "rustc",
          srcs = ["bin/rustc"],
      )

      filegroup(
          name = "rustfmt",
          srcs = ["bin/rustfmt"],
      )

      filegroup(
          name = "cargo",
          srcs = ["bin/cargo"],
      )

      filegroup(
          name = "clippy_driver",
          srcs = ["bin/clippy-driver"],
      )

      filegroup(
          name = "rustc_lib",
          srcs = glob(
              [
                  "bin/*.so",
                  "lib/*.so",
                  "lib/rustlib/*/codegen-backends/*.so",
                  "lib/rustlib/*/codegen-backends/*.dylib",
                  "lib/rustlib/*/bin/rust-lld",
                  "lib/rustlib/*/lib/*.so",
                  "lib/rustlib/*/lib/*.dylib",
              ],
              allow_empty = True,
          ),
      )

      load("@rules_rust//rust:toolchain.bzl", "rust_stdlib_filegroup")
      rust_stdlib_filegroup(
          name = "rust_std",
          srcs = glob(
              [
                  "lib/rustlib/*/lib/*.rlib",
                  "lib/rustlib/*/lib/*.so",
                  "lib/rustlib/*/lib/*.dylib",
                  "lib/rustlib/*/lib/*.a",
                  "lib/rustlib/*/lib/self-contained/**",
              ],
              # Some patterns (e.g. `lib/*.a`) don't match anything, see https://github.com/bazelbuild/rules_rust/pull/245
              allow_empty = True,
          ),
      )

      filegroup(
          name = "rust_doc",
          srcs = ["bin/rustdoc"],
          visibility = ["//visibility:public"],
      )

      load('@rules_rust//rust:toolchain.bzl', 'rust_toolchain')
      rust_toolchain(
          name = "rust_nix_impl",
          rust_doc = ":rust_doc",
          rust_std = ":rust_std",
          rustc = ":rustc",
          rustfmt = ":rustfmt",
          cargo = ":cargo",
          clippy_driver = ":clippy_driver",
          rustc_lib = ":rustc_lib",
          binary_ext = "${binary-ext}",
          staticlib_ext = "${staticlib-ext}",
          dylib_ext = "${dylib-ext}",
          exec_triple = "${build-triple}",
          target_triple = "${target-triple}",
          default_edition = "2021",
          stdlib_linkflags = ["-lpthread", "-ldl"],
      )
      EOF
    '';
  }
