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
        sha256 = "c269fa7f70069180e31aa5982313f5a26838db2a6e5f2e9ecbd5941c8c6ceed0",
        strip_prefix = "rules_nixpkgs-ff70910af286a19dbcc109fe36f2e3cb59da78ff",
        urls = ["https://github.com/tweag/rules_nixpkgs/archive/ff70910af286a19dbcc109fe36f2e3cb59da78ff.tar.gz"],
    )
