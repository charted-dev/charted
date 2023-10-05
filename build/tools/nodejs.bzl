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

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=module-docstring

# to update this, run:
#
#     $ curl https://registry.npmjs.org/typescript | jq '.versions.["<version>"].dist.integrity' | tr -d '"'
#
TYPESCRIPT_INTEGRITY = "sha512-mI4WrpHsbCIcwT9cF4FZvr80QUeKvsUsUvKDoR+X/7XHQH98xYD8YHZg7ANtz2GtZt/CBq2QJ0thkGJMHfqc1w=="
TYPESCRIPT_VERSION = "5.2.2"
NODE_VERSION = "20.7.0"
CHECKSUMS = {
    "linux": {
        "amd64": "a4251c24c6bf6d3bdee4521ca294bc0897a6c466137e02caa2521af5d456f55e",
        "arm64": "c97b51decb0f4a3e8e5bd8cbc6ff43ae4782f2b8b6e3c2b513b77b8f97fffcc5",
    },
    "darwin": {
        "amd64": "ceeba829f44e7573949f2ce2ad5def27f1d6daa55f2860bea82964851fae01bc",
        "arm64": "08aa09792f30a86e8904e334ba6d348ad73e926b5e441ed9abcdcbea651dc926",
    },
    "windows": {
        "amd64": "2b1a117e63f0602bad1e9e31679932b64e9b130a96dc2feb0c367ca816c5a5cb",
    },
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
