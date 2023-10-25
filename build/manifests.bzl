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
CARGO_MANIFESTS = [
    "//:Cargo.toml",
    "//cli:Cargo.toml",
    "//crates/avatars:Cargo.toml",
    "//crates/caching:Cargo.toml",
    "//crates/common:Cargo.toml",
    "//crates/config:Cargo.toml",
    "//crates/database:Cargo.toml",
    "//crates/emails:Cargo.toml",
    "//crates/helm-charts:Cargo.toml",
    "//crates/logging:Cargo.toml",
    "//crates/metrics:Cargo.toml",
    "//crates/openapi/proc-macro:Cargo.toml",
    "//crates/openapi:Cargo.toml",
    "//crates/proc-macros:Cargo.toml",
    "//crates/redis:Cargo.toml",
    "//crates/search/elasticsearch:Cargo.toml",
    "//crates/search/meilisearch:Cargo.toml",
    "//crates/search:Cargo.toml",
    "//crates/sessions/integrations/github:Cargo.toml",
    "//crates/sessions/integrations:Cargo.toml",
    "//crates/sessions/ldap:Cargo.toml",
    "//crates/sessions/local:Cargo.toml",
    "//crates/sessions/passwordless:Cargo.toml",
    "//crates/sessions/zitadel:Cargo.toml",
    "//crates/sessions:Cargo.toml",
    "//crates/storage:Cargo.toml",
    "//features/gc:Cargo.toml",
    "//server/proc-macro:Cargo.toml",
    "//server:Cargo.toml",
    "//testing:Cargo.toml",
    "//tools/devtools:Cargo.toml",
    "//tools/helm-plugin:Cargo.toml",
]

