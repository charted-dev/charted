// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use charted_common::VERSION;
use charted_config::Config;
use once_cell::sync::Lazy;
use utoipa::openapi::{
    external_docs::ExternalDocsBuilder,
    security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    ComponentsBuilder, ContactBuilder, InfoBuilder, LicenseBuilder, OpenApi, OpenApiBuilder, ServerBuilder,
};

pub static API_KEY_SCHEME: Lazy<SecurityScheme> =
    Lazy::new(|| SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey"))));

pub static BEARER_SCHEME: Lazy<SecurityScheme> = Lazy::new(|| {
    let http = HttpBuilder::new()
        .scheme(HttpAuthScheme::Bearer)
        .bearer_format("Bearer")
        .description(Some(
            "JWT-signed session key that is used to safely identify someone for 2 days with a refresh token",
        ))
        .build();

    SecurityScheme::Http(http)
});

pub static BASIC_SCHEME: Lazy<SecurityScheme> = Lazy::new(|| {
    let http = HttpBuilder::new()
        .scheme(HttpAuthScheme::Basic)
        .description(Some(
            "Basic is only meant for testing the API, do not use this for anything else.",
        ))
        .build();

    SecurityScheme::Http(http)
});

/// Creates a new [`OpenAPI`] object.
pub fn openapi() -> OpenApi {
    let config = Config::get();
    let license = LicenseBuilder::new()
        .name("Apache 2.0")
        .url(Some("https://www.apache.org/licenses/LICENSE-2.0"))
        .build();

    let contact = ContactBuilder::new()
        .name(Some("Noelware, LLC."))
        .url(Some("https://noelware.org"))
        .email(Some("team@noelware.org"))
        .build();

    let docs = ExternalDocsBuilder::new()
        .url(format!("https://charts.noelware.org/docs/server/{VERSION}"))
        .description(Some("Main documentation for charted-server"))
        .build();

    let servers = &[ServerBuilder::new()
        .url(format!("http://{}", config.server.addr()))
        .build()];

    let info = InfoBuilder::new()
        .title("charted-server")
        .description(Some(
            "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
        ))
        .version(VERSION)
        .terms_of_service(Some("https://charts.noelware.org/legal/tos"))
        .license(Some(license))
        .contact(Some(contact))
        .build();

    let api_key = &*API_KEY_SCHEME;
    let bearer_scheme = &*BEARER_SCHEME;
    let basic_scheme = &*BASIC_SCHEME;
    let components = ComponentsBuilder::new()
        .security_scheme("ApiKey", api_key.clone())
        .security_scheme("Bearer", bearer_scheme.clone())
        .security_scheme("Basic", basic_scheme.clone())
        .build();

    OpenApiBuilder::new()
        .info(info)
        .external_docs(Some(docs))
        .servers(Some(servers.clone()))
        .components(Some(components))
        .build()
}
