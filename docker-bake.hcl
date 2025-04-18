# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

# The version we are trying to push
variable "PROJECT_VERSION" {}

# this is a special target: https://github.com/docker/metadata-action#bake-definition
target "docker-metadata" {}

target "_common" {
    provenance = true
    context    = "."
    push       = true
}

target "default" {
    inherits = ["debian-x86_64", "alpine-x86_64"]
}

target "debian-x86_64" {
    inherits = ["_common", "docker-metadata"]

    dockerfile = "./distribution/docker/debian.Dockerfile"
    platforms = ["linux/amd64"]
}

target "alpine-x86_64" {
    inherits = ["_common", "docker-metadata"]

    dockerfile = "./distribution/docker/alpine.Dockerfile"
    platforms = ["linux/amd64"]
}

target "debian-aarch64" {
    inherits = ["_common", "docker-metadata"]

    dockerfile = "./distribution/docker/debian.Dockerfile"
    platforms = ["linux/arm64/v8"]
}

target "alpine-aarch64" {
    inherits = ["_common", "docker-metadata"]

    dockerfile = "./distribution/docker/alpine.Dockerfile"
    platforms = ["linux/arm64/v8"]
}
