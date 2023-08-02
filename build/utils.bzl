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

""" bazel is mean """

CARGO_MANIFESTS = ["//{}:Cargo.toml".format(f) for f in [
    "crates/common",
    "crates/config",
    "crates/database",
    "crates/helm-charts",
    "crates/logging",
    "crates/metrics",
    "crates/openapi",
    "crates/redis",
    "crates/search",
    "crates/search/elasticsearch",
    "crates/search/meilisearch",
    "crates/sessions/integrations/github",
    "crates/sessions/integrations",
    "crates/sessions/local",
    "crates/sessions/ldap",
    "crates/sessions",
    "crates/storage",
    "services/emails",
    "tools/helm-plugin",
    "tools/devtools",
    "server",
    "cli",
    "web",
    "",
]]
