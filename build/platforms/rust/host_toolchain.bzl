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
load("@rules_rust//rust/platform:triple.bzl", "get_host_triple")
load("@rules_rust//rust/platform:triple_mappings.bzl", "system_to_dylib_ext", "system_to_staticlib_ext", "system_to_stdlib_linkflags")
load("//:build/utils.bzl", "create_exec_path")

def _detect_os_linkflags(ctx):
    if ctx.os.name.find("windows") != -1:
        return system_to_stdlib_linkflags("windows")

    if ctx.os.name.startswith("mac os"):
        return system_to_stdlib_linkflags("darwin")

    if ctx.os.name.startswith("linux"):
        return system_to_stdlib_linkflags("linux")

    fail("unable to detect os")

def _detect_os_dylib_ext(ctx):
    if ctx.os.name.find("windows") != -1:
        return system_to_dylib_ext("windows")

    if ctx.os.name.startswith("mac os"):
        return system_to_dylib_ext("darwin")

    if ctx.os.name.startswith("linux"):
        return system_to_dylib_ext("linux")

    fail("unable to detect os")

def _detect_os_staticlib_ext(ctx):
    if ctx.os.name.find("windows") != -1:
        return system_to_staticlib_ext("windows")

    if ctx.os.name.startswith("mac os"):
        return system_to_staticlib_ext("darwin")

    if ctx.os.name.startswith("linux"):
        return system_to_staticlib_ext("linux")

    fail("unable to detect os")

def _detect_os_plat(ctx):
    if ctx.os.name.find("windows") != -1:
        return "@platforms//os:windows"

    if ctx.os.name.startswith("mac os"):
        return "@platforms//os:macos"

    if ctx.os.name.startswith("linux"):
        return "@platforms//os:linux"

    fail("unable to detect os")

def _detect_arch_plat(ctx):
    if ctx.os.arch.find("x86_64") != -1 or ctx.os.arch.find("amd64") != -1:
        return "@platforms//cpu:x86_64"

    if ctx.os.arch.find("aarch64") != -1 or ctx.os.arch.find("arm64") != -1:
        return "@platforms//cpu:aarch64"

    fail("unable to detect host arch")

def _rust_configure_host_toolchain(ctx):
    """Configures a host system Rust toolchain, which uses the current host machine's Rust suite of toolchains."""

    # do a empty BUILD.bazel file
    rustc = ctx.which("rustc")
    if rustc == None:
        print("unable to find `rustc`! use the default toolchain that Bazel uses instead")  # buildifier: disable=print
        ctx.file("BUILD.bazel", "")

        return

    res = ctx.execute([create_exec_path(ctx, rustc), "--print", "sysroot"])
    if res.return_code:
        fail("unable to print sysroot with given rustc [%s]" % rustc)

    sysroot = res.stdout.removesuffix("\n")

    # symlink {sysroot}/bin ~> @rust_host//:bin/<binary>
    binaries = ["rustdoc", "rustfmt", "clippy-driver", "cargo", "rustc"]
    for bin in binaries:
        ctx.symlink("%s/bin/%s" % (sysroot, bin), "bin/%s" % bin)

    # symlink {sysroot}/lib ~> @rust_host//:lib
    host_triple = get_host_triple(ctx)
    ctx.symlink("%s/lib" % sysroot, "lib")

    # write BUILD.bazel
    os, arch, staticlib_ext, dylib_ext, linkflags = _detect_os_plat(ctx), _detect_arch_plat(ctx), _detect_os_staticlib_ext(ctx), _detect_os_dylib_ext(ctx), _detect_os_linkflags(ctx)
    ctx.file("BUILD.bazel", content = """load("@rules_rust//rust:toolchain.bzl", "rust_toolchain", "rust_stdlib_filegroup")

package(default_visibility = ["//visibility:public"])

exports_files(glob(["bin/**"]))

filegroup(
    name = "rustc_lib",
    srcs = glob(
        [
            "bin/*.so",
            "lib/*.so",
            "lib/rustlib/*/codegen-backends/*.so",
            "lib/rustlib/*/codegen-backends/*.dylib",
            "lib/rustlib/*/bin/rust-lld",
            "lib/rustlib/*/lib/*.so",
            "lib/rustlib/*/lib/*.dylib",
        ],
        allow_empty = True
    ),
)

rust_stdlib_filegroup(
    name = "rust_std",
    srcs = glob(
        [
            "lib/rustlib/*/lib/*.rlib",
            "lib/rustlib/*/lib/*.so",
            "lib/rustlib/*/lib/*.dylib",
            "lib/rustlib/*/lib/*.a",
            "lib/rustlib/*/lib/self-contained/**",
        ],

        # Some patterns (e.g. `lib/*.a`) don't match anything, see https://github.com/bazelbuild/rules_rust/pull/245
        allow_empty = True,
    )
)

rust_toolchain(
    name = "rust_toolchain_impl",
    binary_ext = "{binary_ext}",
    cargo = ":bin/cargo",
    clippy_driver = ":bin/clippy-driver",
    default_edition = "2021",
    dylib_ext = "{dylib_ext}",
    exec_triple = "{triple}",
    rust_doc = ":bin/rustdoc",
    rust_std = ":rust_std",
    rustc = ":bin/rustc",
    staticlib_ext = "{staticlib_ext}",
    stdlib_linkflags = {stdlib_linkflags_serialized},
    target_triple = "{triple}",
    visibility = ["//:__pkg__"],
)

toolchain(
    name = "rust_toolchain",
    exec_compatible_with = ["{os_plat}", "{arch_plat}"],
    target_compatible_with = ["{os_plat}", "{arch_plat}"],
    toolchain = ":rust_toolchain_impl",
    toolchain_type = "@rules_rust//rust:toolchain_type",
)
""".format(
        binary_ext = ".exe" if ctx.os.name.find("windows") != -1 else "",
        triple = host_triple.str,
        os_plat = os,
        arch_plat = arch,
        dylib_ext = dylib_ext,
        staticlib_ext = staticlib_ext,
        stdlib_linkflags_serialized = json.encode(linkflags),
    ))

rust_configure_host_toolchain = repository_rule(
    implementation = _rust_configure_host_toolchain,
)
