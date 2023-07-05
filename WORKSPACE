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

http_archive(
    name = "rules_rust",
    sha256 = "0c2ff9f58bbd6f2a4fc4fbea3a34e85fe848e7e4317357095551a18b2405a01c",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.25.0/rules_rust-v0.25.0.tar.gz"],
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
    name = "aspect_rules_esbuild",
    sha256 = "2ea31bd97181a315e048be693ddc2815fddda0f3a12ca7b7cc6e91e80f31bac7",
    strip_prefix = "rules_esbuild-0.14.4",
    url = "https://github.com/aspect-build/rules_esbuild/releases/download/v0.14.4/rules_esbuild-v0.14.4.tar.gz",
)

# Load up rules_rust dependencies
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains(edition = "2021", versions = ["1.70.0"])

load("@rules_rust//cargo:defs.bzl", "cargo_bootstrap_repository")
cargo_bootstrap_repository(
    cargo_lockfile = "//:Cargo.lock",
    cargo_toml = "//:Cargo.toml",
    name = "charted-server"
)

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")
crates_repository(
    manifests = [
        "//:Cargo.toml",
        "//:cli/Cargo.toml",
        "//:server/Cargo.toml"
    ],

    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.bzl.lock",
    name = "crate_index"
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()

# Load up rules_js/ts/esbuild dependencies next
load("@aspect_rules_js//js:repositories.bzl", "rules_js_dependencies")
rules_js_dependencies()

load("@aspect_rules_ts//ts:repositories.bzl", "rules_ts_dependencies")
rules_ts_dependencies(ts_version = "5.1.6")

load("@aspect_rules_esbuild//esbuild:dependencies.bzl", "rules_esbuild_dependencies")
rules_esbuild_dependencies()

load("@rules_nodejs//nodejs:repositories.bzl", "nodejs_register_toolchains", "node_repositories")
node_repositories(
    name = "nodejs_repo",
    node_version = "20.3.1",
    node_repositories = {
        "20.3.1-linux_amd64": ("node-v20.3.1-linux-x64.tar.xz", "node-v20.3.1-linux-x64", "ecfe263dbd9c239f37b5adca823b60be1bb57feabbccd25db785e647ebc5ff5e"),
        "20.3.1-linux_arm64": ("node-v20.3.1-linux-arm64.tar.gz", "node-v20.3.1-linux-arm64", "555b5c521e068acc976e672978ba0f5b1a0c030192b50639384c88143f4460bc"),
        "20.3.1-darwin_amd64": ("node-v20.3.1-darwin-x64.tar.gz", "node-v20.3.1-darwin-x64", "3040210287a0b8d05af49f57de191afa783e497abbb10c340bae9158cb51fdd4"),
        "20.3.1-darwin_arm64": ("node-v20.3.1-darwin-arm64.tar.gz", "node-v20.3.1-darwin-arm64", "2ccb24e9211f4d17d8d8cfc0ea521198bb6a54e2f779f8feda952dbd3bb651ac"),
        "20.3.1-windows_amd64": ("node-v20.3.1-win-x64.zip", "node-v20.3.1-win-x64", "145bd2f79eaa50b76559bd78266f4585e57b88dbb94613698a9514a601f84e7f")
    }
)

nodejs_register_toolchains(name = "nodejs", node_version = "20.3.1")
