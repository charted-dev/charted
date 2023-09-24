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
    "1.72.0",
    "nightly/2023-09-03",
]

def charted_rust_repositories():
    http_archive(
        name = "rules_rust",
        sha256 = "db89135f4d1eaa047b9f5518ba4037284b43fc87386d08c1d1fe91708e3730ae",
        urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.27.0/rules_rust-v0.27.0.tar.gz"],
    )