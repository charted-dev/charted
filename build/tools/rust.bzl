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
        strip_prefix = "rules_rust-5ceebc08a482536a723e4414e327d8ada48b7f4d",
        sha256 = "2aea2abebda8835febf671ff38689d06b3eeb0faa1612eccf1548340203989a5",
        urls = ["https://github.com/bazelbuild/rules_rust/archive/5ceebc08a482536a723e4414e327d8ada48b7f4d.tar.gz"],
    )
