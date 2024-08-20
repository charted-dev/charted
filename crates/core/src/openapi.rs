// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::VERSION;
use std::sync::LazyLock;
use utoipa::openapi::{
    external_docs::ExternalDocsBuilder,
    security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Components, ComponentsBuilder, ContactBuilder, Info, InfoBuilder, License, OpenApi, OpenApiBuilder,
};

#[allow(clippy::incompatible_msrv)]
pub static COMPONENTS: LazyLock<Components> = LazyLock::new(|| {
    ComponentsBuilder::new()
        .security_scheme(
            "ApiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey"))),
        )
        .security_scheme("Bearer", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
        .security_scheme("Basic", SecurityScheme::Http(Http::new(HttpAuthScheme::Basic)))
        .build()
});

fn info() -> Info {
    InfoBuilder::new()
        .title("charted-server")
        .version(crate::VERSION)
        .description(Some("🐻‍❄️📦 Reliable, free, and open Helm chart registry in Rust"))
        .terms_of_service(Some("https://charts.noelware.org/legal/tos"))
        .license(Some(License::new("Apache-2.0")))
        .contact(Some(
            ContactBuilder::new()
                .name(Some("Noelware, LLC."))
                .email(Some("team@noelware.org"))
                .url(Some("https://noelware.org"))
                .build(),
        ))
        .build()
}

/// Returns a [`OpenApi`] document that documents the REST API for charted-server. This only
/// includes the schemas, [info][Info], and security schemes. The `charted_server` crate will
/// extend this.
pub fn document() -> OpenApi {
    OpenApiBuilder::new()
        .info(info())
        .external_docs(Some(
            ExternalDocsBuilder::new()
                .url(format!(
                    "https://charts.noelware.org/docs/server/{VERSION}/api/reference"
                ))
                .build(),
        ))
        .build()
}
