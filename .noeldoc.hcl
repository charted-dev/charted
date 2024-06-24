# 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
# Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

noeldoc {
    version     = ">=0.1"
    experiments = ["dockerRunner"]
}

# The Markdown source files will be avaliable as its AST representation, where the
# documentation site (`charted-dev/docs`) will pull from the following branches:
#
#     * main
#     * each release (v0.1.0-beta and above)
extension "noeldoc/markdown" {
    version = "0.1.0"
    srcs    = glob(["${cwd}/docs/**/*.md"])
}

# We generate the extension schema for the configuration file and CLI commands.
extension "noeldoc/rustdoc" {
    version = "0.1.0"
    cargo {
        toml   = "${cwd}/Cargo.toml"
        crates = ["charted-config", "charted-cli"]
    }
}
