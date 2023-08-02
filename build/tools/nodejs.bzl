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

NODE_VERSION = "20.5.0"
CHECKSUMS = {
    "linux": {
        "amd64": "c12ee9efe21f3ff9909fbf5d7d3780f16c86fad411f13d715016646c766e8213",
        "arm64": "afddd830662bdc71f37d39d6cd74104acc663ecd6bbe0fd9264c581ee4f2559b",
    },
    "darwin": {
        "amd64": "3da7e64ac76309cbbb25524bae75cd335fed2795fcbd4f55e3162bcbcec18176",
        "arm64": "56d29a7c620415164e6226804cc1eb8eb7b05ea3123b60c86393fabb551bd5ea",
    },
    "windows": {
        "amd64": "604e7308bb314fb8c27979929db2877940ce27a489ccafc6367f439943730b32",
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
