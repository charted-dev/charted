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
        sha256 = "f0024fda8e0c61a56b2090964dd24248e6b620e61b77292a946a527b89fa7f7d",
        strip_prefix = "rules_nixpkgs-3a8141aec5d27a9aee358faa737aa4405dc04022",
        urls = ["https://github.com/tweag/rules_nixpkgs/archive/3a8141aec5d27a9aee358faa737aa4405dc04022.tar.gz"],
    )
