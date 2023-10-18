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
load("@crate_index//:defs.bzl", "aliases")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc_test", "rust_library", "rust_proc_macro", "rust_test")

_DEFAULT_RUST_EDITION = "2021"

def rust_project(
        name,
        srcs = [],
        deps = [],
        external_data = [],
        proc_macro = False,
        proc_macro_deps = [],
        include_tests = False,
        include_doctests = False,
        test_deps = [],
        is_binary = False,
        build_script = False,
        build_script_data = [],
        build_script_deps = [],
        **kwargs):
    """A common `rust_project` macro to help aid repeating ourselves.

    Args:
        name: The name of the project.
        srcs: Extra sources to include in `rust_library`
        deps: A list of dependencies to use in the `rust_library`, `rust_binary` (if enabled), and `rust_test` (if enabled) macro(s).
        external_data: List of targets to use to embed data into `rust_library`.
        proc_macro: Builds this project as a proc-macro.
        proc_macro_deps: A list of proc-macro related dependencies to use.
        include_tests: If the `rust_test` macro should be included. This will always be `{name}_test` when used with
            the `bazel test` command.
        include_doctests: If doctests should be enable in this project.
        test_deps: Any other external dependencies that should be only in tests and not leaked into the main project scope.
        is_binary: Whether if the `rust_binary` should be included for this project, for most Bazel packages, this will use
            the default (False) since the CLI and the OpenAPI codegen scripts are the only binaries that should be
            allowed.
        build_script: Whether if the `cargo_build_script` macro should be included. This will always be `{name}_buildscript`
            when used.
        build_script_data: Some buildscripts might require to read files or execute CLI commands, this is where you should
            include data that should be used when the Cargo buildscript is executed.
        build_script_deps: List of dependencies to use only for the buildscript that is not leaked in the main/test project scope.
        **kwargs: Extra arguments to append in `rust_library`
    """

    extra_flags = ["--cfg=bazel"]
    if kwargs.get("rustc_flags") != None:
        flags = kwargs.pop("rustc_flags")
        extra_flags = extra_flags + flags

    if proc_macro:
        rust_proc_macro(
            name = "charted_%s" % name,
            aliases = aliases(),
            srcs = native.glob(["src/**/*.rs"], exclude = ["src/main.rs"]) + srcs,
            deps = deps,
            visibility = ["//visibility:public"],
            rustc_flags = extra_flags,
            edition = _DEFAULT_RUST_EDITION,
            **kwargs
        )
    else:
        rust_library(
            # We need it as charted_<name> so it can be referenced with Bazel
            # without using 'extern crate {name}'!
            name = "charted_%s" % name,
            aliases = aliases(),
            data = external_data,
            srcs = native.glob(["src/**/*.rs"], exclude = ["src/main.rs"]) + srcs,
            deps = deps,
            proc_macro_deps = proc_macro_deps,
            visibility = ["//visibility:public"],
            rustc_flags = extra_flags,
            edition = _DEFAULT_RUST_EDITION,
            **kwargs
        )

    if include_tests:
        rust_test(
            name = "tests",
            srcs = native.glob(["src/**/*.rs"], exclude = ["src/main.rs"]),
            deps = [":charted_{name}".format(name = name)] + deps + test_deps,
            compile_data = external_data,
            proc_macro_deps = proc_macro_deps,
            rustc_flags = extra_flags,
        )

        # rust_test_suite(
        #     name = "integ_tests",
        #     srcs = native.glob(["tests/**/*.rs"]),
        #     deps = [":charted_{name}".format(name = name)] + deps + test_deps,
        #     compile_data = external_data + [":Cargo.toml"],
        #     proc_macro_deps = proc_macro_deps,
        #     rustc_flags = extra_flags,
        # )

    if include_doctests:
        rust_doc_test(
            name = "doctests",
            crate = ":charted_{name}".format(name = name),
        )

    if is_binary:
        rust_binary(
            name = name,
            srcs = ["src/main.rs"],
            deps = [":charted_{name}".format(name = name)] + deps,
            rustc_flags = ["-C", "incremental=true"] + extra_flags,
            visibility = ["//visibility:public"],
            edition = _DEFAULT_RUST_EDITION,
        )

    if build_script:
        cargo_build_script(
            name = "buildscript",
            srcs = ["build.rs"],
            data = build_script_data,
            deps = build_script_deps,
        )
