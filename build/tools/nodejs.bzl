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
NODE_VERSION = "20.8.0"
CHECKSUMS = {
    "linux:amd64": "66056a2acc368db142b8a9258d0539e18538ae832b3ccb316671b0d35cb7c72c",
    "linux:arm64": "ec2d98894d58d07260e61e6a70b88cabea98292f0b2801cbeebd864d242e1087",
    "darwin:amd64": "a6f6b573ea656c149956f69f35e04ebb242b945d59972bea2e96a944bbf50ad1",
    "darwin:arm64": "cbcb7fdbcd9341662256df5e4488a0045242f87382879242093e0f0699511abc",
    "windows:amd64": "6afd5a7aa126f4e255f041de66c4a608f594190d34dcaba72f7b348d2410ca66",
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
