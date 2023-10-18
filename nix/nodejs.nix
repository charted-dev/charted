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
# adapted from tweag/rules_nixpkgs to include pnpm and use the latest Node.js version
#
# original template: https://github.com/tweag/rules_nixpkgs/blob/master/toolchains/nodejs/nodejs.bzl#L24-L53
let
  pkgs = import <nixpkgs> {
    config = {};
    overlays = [];
    system = builtins.currentSystem;
  };

  nodejs = pkgs.nodejs_20;
in
  pkgs.buildEnv {
    extraOutputsToInstall = ["out" "bin" "lib"];
    name = "nodejs-bazel-toolchain";
    paths = with pkgs; [
      nodePackages.pnpm
      nodejs
    ];

    postBuild = ''
      touch $out/ROOT

      cat <<EOF > $out/BUILD
      load("@rules_nodejs//nodejs:toolchain.bzl", "node_toolchain")
      package(default_visibility = ["//visibility:public"])
      filegroup(
          name = "nodejs",
          srcs = ["bin/node"],
      )
      node_toolchain(
          name = "nodejs_nix_impl",
          target_tool = ":nodejs",
      )
      EOF
    '';
  }
