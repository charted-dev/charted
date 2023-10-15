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

load("//:build/tools/nixpkgs.bzl", "charted_nixpkgs_repositories")

charted_nixpkgs_repositories()

load("@io_tweag_rules_nixpkgs//nixpkgs:repositories.bzl", "rules_nixpkgs_dependencies")

rules_nixpkgs_dependencies()

load(
    "@io_tweag_rules_nixpkgs//nixpkgs:nixpkgs.bzl",
    "nixpkgs_cc_configure",
    "nixpkgs_flake_package",
    "nixpkgs_git_repository",
    "nixpkgs_package",
    "nixpkgs_rust_configure",
)

nixpkgs_git_repository(
    name = "nixpkgs",

    # TODO(@auguwu): should we use nixpkgs-unstable? do we pin to a tag?
    #                also, we should also keep this the same as the one
    #                in the Nix flake.
    revision = "nixpkgs-unstable",
)

# makes our nix flake available in the Bazel workspace
nixpkgs_flake_package(
    name = "flake",
    nix_flake_file = "//:flake.nix",
    nix_flake_lock_file = "//:flake.lock",
)

# configures the CC toolchain we will use, we opt to use Clang with
# the mold linker.
nixpkgs_cc_configure(
    name = "nixpkgs_config_cc",
    nix_file_content = "(import <nixpkgs> {}).clang_16",
    repository = "@nixpkgs",
)

# configures the Rust toolchain, which will reflect what the rust-toolchain.toml file
# uses, which is stable Rust.
nixpkgs_rust_configure(
    name = "nix_rust",
    repository = "@nixpkgs",
)

# This will configure the OpenSSL static library that NixOS can link `charted-cli`
# and `charted-helm-plugin` with.
nixpkgs_package(
    name = "openssl-static",
    nix_file = "//:nixos/openssl.nix",
    repository = "@nixpkgs",
)

load("//:build/tools/cc.bzl", "charted_cc_repositories")

charted_cc_repositories()

load("@rules_cc//cc:repositories.bzl", "rules_cc_dependencies", "rules_cc_toolchains")

rules_cc_dependencies()

rules_cc_toolchains()

load(
    "//:build/tools/rust.bzl",
    "RUST_EDITION",
    "RUST_VERSIONS",
    "charted_rust_repositories",
)

charted_rust_repositories()

# load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains", "rust_repository_set")

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

load("//build/platforms:rust/host_toolchain.bzl", "rust_configure_host_toolchain")

rust_configure_host_toolchain(name = "rust_host_toolchain")

rust_register_toolchains(
    edition = RUST_EDITION,
    versions = RUST_VERSIONS,
)

# [
#     rust_repository_set(
#         name = "rust-%s-%s%s" % (
#             os,
#             arch,
#             ("-%s" % component) if component != None else "",
#         ),
#         edition = RUST_EDITION,
#         exec_triple = exec_triple,
#         versions = RUST_VERSIONS,
#     )
#     for (os, arch, exec_triple, component) in [
#         ("linux", "x86_64", "x86_64-unknown-linux-gnu", None),
#         ("linux", "x86_64", "x86_64-unknown-linux-musl", "musl"),
#         ("linux", "aarch64", "aarch64-unknown-linux-musl", "musl"),
#         ("linux", "aarch64", "aarch64-unknown-linux-gnu", None),
#         ("darwin", "x86_64", "x86_64-apple-darwin", None),
#         ("darwin", "aarch64", "aarch64-apple-darwin", None),
#         ("windows", "x86_64", "x86_64-pc-windows-msvc", None),
#     ]
# ]

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

load("@rules_rust//cargo:defs.bzl", "cargo_bootstrap_repository")

cargo_bootstrap_repository(
    name = "charted-server",
    cargo_lockfile = "//:Cargo.lock",
    cargo_toml = "//:Cargo.toml",
)

load("//thirdparty/crates:crates.bzl", "crate_repositories")

crate_repositories()

load("//:build/tools/protobuf.bzl", "charted_protobuf_repositories")

charted_protobuf_repositories()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

load(
    "//:build/tools/nodejs.bzl",
    "CHECKSUMS",
    "NODE_VERSION",
    "TYPESCRIPT_INTEGRITY",
    "TYPESCRIPT_VERSION",
    "charted_nodejs_repositories",
    "create_tuple",
    "format_key",
)

charted_nodejs_repositories()

load("@aspect_rules_js//js:repositories.bzl", "rules_js_dependencies")

rules_js_dependencies()

load("@aspect_rules_ts//ts:repositories.bzl", "rules_ts_dependencies")

rules_ts_dependencies(
    ts_integrity = TYPESCRIPT_INTEGRITY,
    ts_version = TYPESCRIPT_VERSION,
)

load("@rules_nodejs//nodejs:repositories.bzl", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "nodejs",
    node_repositories = {
        format_key("linux", "amd64"): create_tuple(
            "linux",
            "x64",
            "tar.xz",
            CHECKSUMS["linux:amd64"],
        ),
        format_key("linux", "arm64"): create_tuple(
            "linux",
            "arm64",
            "tar.xz",
            CHECKSUMS["linux:arm64"],
        ),
        format_key("darwin", "amd64"): create_tuple(
            "darwin",
            "x64",
            "tar.gz",
            CHECKSUMS["darwin:amd64"],
        ),
        format_key("darwin", "arm64"): create_tuple(
            "darwin",
            "arm64",
            "tar.gz",
            CHECKSUMS["darwin:arm64"],
        ),
        format_key("windows", "amd64"): create_tuple(
            "windows",
            "x64",
            "zip",
            CHECKSUMS["windows:amd64"],
        ),
    },
    node_version = NODE_VERSION,
)

load("@aspect_rules_js//npm:npm_import.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm",
    data = ["//web:package.json"],
    npmrc = "//:.npmrc",
    pnpm_lock = "//web:pnpm-lock.yaml",
    update_pnpm_lock = True,
    verify_node_modules_ignored = "//:.bazelignore",
)

npm_translate_lock(
    name = "types_npm",
    data = ["//types/js:package.json"],
    npmrc = "//:.npmrc",
    pnpm_lock = "//types/js:pnpm-lock.yaml",
    update_pnpm_lock = True,
    verify_node_modules_ignored = "//:.bazelignore",
)

load("@npm//:repositories.bzl", "npm_repositories")

npm_repositories()

load("@types_npm//:repositories.bzl", types_repositories = "npm_repositories")

types_repositories()

load("//:build/tools/pkg.bzl", "charted_pkg_repositories")

charted_pkg_repositories()

load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()
