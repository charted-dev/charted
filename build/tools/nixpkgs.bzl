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
        sha256 = "5da887427200c97ac0e24faa552b95f78337e640e69374c194fc181231711d9e",
        strip_prefix = "rules_nixpkgs-29eaa0cd3fdf71af089cc4b9bbe1abb91ff04962",
        urls = ["https://github.com/tweag/rules_nixpkgs/archive/29eaa0cd3fdf71af089cc4b9bbe1abb91ff04962.tar.gz"],
    )
