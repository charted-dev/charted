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
use utoipa::openapi::{
    external_docs::ExternalDocsBuilder, ContactBuilder, InfoBuilder, LicenseBuilder, OpenApi, OpenApiBuilder,
    ServerBuilder,
};

/// Creates a new [`OpenAPI`] object.
pub fn openapi() -> OpenApi {
    let config = Config::get();
    let info = InfoBuilder::new()
        .title("charted-server")
        .version(VERSION)
        .terms_of_service(Some("https://charts.noelware.org/legal/tos"))
        .license(Some(
            LicenseBuilder::new()
                .name("Apache 2.0")
                .url(Some("https://www.apache.org/licenses/LICENSE-2.0"))
                .build(),
        ))
        .contact(Some(
            ContactBuilder::new()
                .name(Some("Noelware, LLC."))
                .url(Some("https://noelware.org"))
                .email(Some("team@noelware.org"))
                .build(),
        ))
        .description(Some(
            "🐻‍❄️📦 Free, open source, and reliable Helm Chart registry made in Rust",
        ))
        .build();

    OpenApiBuilder::new()
        .info(info)
        .external_docs(Some(
            ExternalDocsBuilder::new()
                .url(format!("https://charts.noelware.org/docs/server/{VERSION}"))
                .build(),
        ))
        .servers(Some([ServerBuilder::new()
            .url(format!("http://{}", config.server.addr()))
            .build()]))
        .build()
}
