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
load("@bazel_tools//tools/build_defs/cc:action_names.bzl", "CPP_LINK_EXECUTABLE_ACTION_NAME")
load("@bazel_tools//tools/cpp:cc_toolchain_config_lib.bzl", "action_config", "tool")

# buildifier: disable=function-docstring
def _define_musl_toolchain_impl(ctx):
    return cc_common.create_cc_toolchain_config_info(
        ctx = ctx,
        toolchain_identifier = "musl",
        host_system_name = "local",
        target_system_name = "linux-musl",
        target_cpu = "haswell",
        target_libc = "musl",
        compiler = "clang",
        abi_version = "unknown",
        abi_libc_version = "unknown",
        action_configs = [
            action_config(
                action_name = CPP_LINK_EXECUTABLE_ACTION_NAME,
                enabled = True,
                tools = [
                    tool(path = "/usr/local/bin/x86_64-linux-musl-ld"),
                ],
            ),
        ],
    )

define_musl_toolchain = rule(
    implementation = _define_musl_toolchain_impl,
    attrs = {},
    provides = [CcToolchainConfigInfo],
)
