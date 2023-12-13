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

# buildifier: disable=module-docstring
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def charted_nixpkgs_repositories():
    http_archive(
        name = "io_tweag_rules_nixpkgs",
        sha256 = "25aa3eab0fa96deab119b8cf114e4f7cfd9215beb9ad7094f30f9c57f9f02310",
        strip_prefix = "rules_nixpkgs-1ad0bd952d1faa2943944d23bb9795c9fff49bc7",
        urls = ["https://github.com/tweag/rules_nixpkgs/archive/1ad0bd952d1faa2943944d23bb9795c9fff49bc7.tar.gz"],
    )
