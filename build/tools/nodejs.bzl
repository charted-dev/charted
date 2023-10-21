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

TYPESCRIPT_INTEGRITY = "sha512-mI4WrpHsbCIcwT9cF4FZvr80QUeKvsUsUvKDoR+X/7XHQH98xYD8YHZg7ANtz2GtZt/CBq2QJ0thkGJMHfqc1w=="
TYPESCRIPT_VERSION = "5.2.2"
NODE_VERSION = "20.8.1"
CHECKSUMS = {
    "linux:amd64": "44096f6276cf735f3b25f47ffaaa1629b0abad4d9932c3a77d9dcdc743a3ff92",
    "linux:arm64": "fec6edefa7ff959b29c7887735582ff2a2211b36a65a539da0f37db6797b7cff",
    "darwin:amd64": "92b00b357c311eb45dd86516b032d80c63894aa069821c3ae3c8b3bbd00fdb9a",
    "darwin:arm64": "5451f3651c89cd8f224e74961c84e68f4c8d63fe288431a3223b0465cc8b961e",
    "windows:amd64": "90b27dab351a582edd3a8de2e8aaa80d95c41f19fe92ebbef83b9a45bac95d00",
}

def format_key(os, arch):
    return "%s-%s_%s" % (NODE_VERSION, os, arch)

def create_tuple(os, arch, ext, checksum):
    return (
        "node-v%s-%s-%s.%s" % (NODE_VERSION, os, arch, ext),
        "node-v%s-%s-%s" % (NODE_VERSION, os, arch),
        checksum,
    )

def charted_nodejs_repositories():
    http_archive(
        name = "aspect_rules_js",
        sha256 = "77c4ea46c27f96e4aadcc580cd608369208422cf774988594ae8a01df6642c82",
        strip_prefix = "rules_js-1.32.2",
        url = "https://github.com/aspect-build/rules_js/releases/download/v1.32.2/rules_js-v1.32.2.tar.gz",
    )

    http_archive(
        name = "aspect_rules_ts",
        sha256 = "8aabb2055629a7becae2e77ae828950d3581d7fc3602fe0276e6e039b65092cb",
        strip_prefix = "rules_ts-2.0.0",
        url = "https://github.com/aspect-build/rules_ts/releases/download/v2.0.0/rules_ts-v2.0.0.tar.gz",
    )
