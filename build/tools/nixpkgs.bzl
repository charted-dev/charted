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
        sha256 = "d4a8c10121ec7494402a0ae8c1a896ced20d4bef4485b107e37f5331716c3626",
        strip_prefix = "rules_nixpkgs-244ae504d3f25534f6d3877ede4ee50e744a5234",
        urls = ["https://github.com/tweag/rules_nixpkgs/archive/244ae504d3f25534f6d3877ede4ee50e744a5234.tar.gz"],
    )
