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

workspace(name = "org_noelware_charted_server")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("//:build/utils.bzl", "CARGO_MANIFESTS")

http_archive(
    name = "rules_rust",
    sha256 = "9d04e658878d23f4b00163a72da3db03ddb451273eb347df7d7c50838d698f49",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.26.0/rules_rust-v0.26.0.tar.gz"],
)

http_archive(
    name = "aspect_rules_js",
    sha256 = "71895e99936ab4cdb2c2ed6f076134cf5799c478c33ae3fa934f279b585a9b38",
    strip_prefix = "rules_js-1.29.0",
    url = "https://github.com/aspect-build/rules_js/releases/download/v1.29.0/rules_js-v1.29.0.tar.gz",
)

http_archive(
    name = "aspect_rules_ts",
    sha256 = "2bf5e2398713561ddaaaed8385dd6cee1bb21fe7856a5aac57b9e99ebf0291e2",
    strip_prefix = "rules_ts-1.4.4",
    url = "https://github.com/aspect-build/rules_ts/releases/download/v1.4.4/rules_ts-v1.4.4.tar.gz",
)

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "278b7ff5a826f3dc10f04feaf0b70d48b68748ccd512d7f98bf442077f043fe3",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.41.0/rules_go-v0.41.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.41.0/rules_go-v0.41.0.zip",
    ],
)

http_archive(
    name = "bazel_gazelle",
    sha256 = "29218f8e0cebe583643cbf93cae6f971be8a2484cdcfa1e45057658df8d54002",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.32.0/bazel-gazelle-v0.32.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.32.0/bazel-gazelle-v0.32.0.tar.gz",
    ],
)

http_archive(
    name = "rules_proto",
    sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
    strip_prefix = "rules_proto-5.3.0-21.7",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
    ],
)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.20.6")

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

gazelle_dependencies()

load("//services/search-indexer:deps.bzl", "go_dependencies")

# gazelle:repository_macro services/search-indexer/deps.bzl%go_dependencies
go_dependencies()

# Load up rules_rust dependencies
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.71.0",
        "nightly/2023-07-28",
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

load("@rules_rust//cargo:defs.bzl", "cargo_bootstrap_repository")

cargo_bootstrap_repository(
    name = "charted-server",
    cargo_lockfile = "//:Cargo.lock",
    cargo_toml = "//:Cargo.toml",
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.bzl.lock",
    manifests = CARGO_MANIFESTS,
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Load up rules_js/ts dependencies next
load("@aspect_rules_js//js:repositories.bzl", "rules_js_dependencies")

rules_js_dependencies()

load("@aspect_rules_ts//ts:repositories.bzl", "rules_ts_dependencies")

rules_ts_dependencies(ts_version = "5.1.6")

load("@rules_nodejs//nodejs:repositories.bzl", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "nodejs",
    node_repositories = {
        "20.4.0-linux_amd64": ("node-v20.4.0-linux-x64.tar.xz", "node-v20.4.0-linux-x64", "6b49a007f409fb7620350285cfc909fbc909604fd0ff5a87a1730365514b3712"),
        "20.4.0-linux_arm64": ("node-v20.4.0-linux-arm64.tar.gz", "node-v20.4.0-linux-arm64", "6ed340475a8bd5db5f04fe943b8fb89b7b2a8fd919f91217c6386dfa59865ba3"),
        "20.4.0-darwin_amd64": ("node-v20.4.0-darwin-x64.tar.gz", "node-v20.4.0-darwin-x64", "fe765474a8651b85cee04a64e8473089196b922a36621f464a985a5f4891a054"),
        "20.4.0-darwin_arm64": ("node-v20.4.0-darwin-arm64.tar.gz", "node-v20.4.0-darwin-arm64", "34f51397b6aad957b1a8eb70d13da5baf357ead124c1e429a7e939aa61266c06"),
        "20.4.0-windows_amd64": ("node-v20.4.0-win-x64.zip", "node-v20.4.0-win-x64", "91a51aaa9152db510704b4274cffd84c6e3572e1678e055e0d9c5cf7951ebc2a"),
    },
    node_version = "20.4.0",
)

load("@aspect_rules_js//npm:npm_import.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm",
    data = ["//web:package.json"],
    npmrc = "//web:.npmrc",
    pnpm_lock = "//web:pnpm-lock.yaml",
    update_pnpm_lock = True,
    verify_node_modules_ignored = "//:.bazelignore",
)

load("@npm//:repositories.bzl", "npm_repositories")

npm_repositories()
