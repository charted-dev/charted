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

RUST_EDITION = "2021"
RUST_VERSIONS = [
    "1.73.0",
    "nightly/2023-10-12",
]

def charted_rust_repositories():
    # TODO(@auguwu): switch to newer rules-rust once commit 3a013f8bf11cfda776fce91e3dc0cee387d8c001 is in a release
    http_archive(
        name = "rules_rust",
        strip_prefix = "rules_rust-f11c529e802ee1a4537634d4a175bd6658dd60be",
        sha256 = "0dd74fde5810b2a8e47d6fdead0dd239fb6f976fae555996c21c119ce4334941",
        urls = ["https://github.com/bazelbuild/rules_rust/archive/f11c529e802ee1a4537634d4a175bd6658dd60be.tar.gz"],
    )
