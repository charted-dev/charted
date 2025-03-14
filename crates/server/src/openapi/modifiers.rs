// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

use charted_core::api::Version;
use std::collections::BTreeMap;
use utoipa::{
    Modify,
    openapi::{
        ComponentsBuilder, OpenApi,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

const BEARER_SCHEME_DESCRIPTION: &str = "Signed token by the server to safely authenticate yourself";
const BASIC_SCHEME_DESCRIPTION: &str = "Allows to use basic authentication (base64 of `username:password`). Do note that some instances might have this disabled and it isn't recommended to be used in production environments.";

/// [`Modify`] implementation to iterate over each route and remove the `/{version}`
/// prefix on the default API version.
pub struct UpdatePathsToIncludeDefaultVersion;

impl Modify for UpdatePathsToIncludeDefaultVersion {
    fn modify(&self, openapi: &mut OpenApi) {
        let default_api_version = Version::default();
        let should_match = format!("/{version}", version = default_api_version.as_str());
        let mut paths = openapi.paths.paths.clone();

        for (mut path, item) in paths
            .clone()
            .into_iter()
            .filter(|(key, _)| key.starts_with(&should_match))
        {
            path = path.trim_start_matches(&should_match).to_owned();
            if path.is_empty() {
                path = "/".into();
            }

            BTreeMap::insert(&mut paths, path, item);
        }

        openapi.paths.paths.extend(paths);
    }
}

/// [`Modify`] implementation to include error-proned schemas.
pub struct IncludeErrorProneDatatypes;

// TODO(@auguwu): don't know why `DateTime` and `serde::Duration` won't compile
//                into the `components(schemas())` directive in the `#[openapi]`
//                attribute:
//
// error: expected expression, found `,`
//   --> crates/server/src/openapi.rs:28:10
//    |
// 28 | #[derive(OpenApi)]
//    |          ^^^^^^^ expected expression
//    |
//    = note: this error originates in the derive macro `OpenApi` (in Nightly builds, run
// with -Z macro-backtrace for more info)
//
// error: expected one of `.`, `;`, `?`, `else`, or an operator, found `)`
//   --> crates/server/src/openapi.rs:28:10
//    |
// 28 | #[derive(OpenApi)]
//    |          ^^^^^^^ expected one of `.`, `;`, `?`, `else`, or an operator
//    |
//    = note: this error originates in the derive macro `OpenApi` (in Nightly builds, run
// with -Z macro-backtrace for more info)
//
// error: proc-macro derive produced unparsable tokens
//   --> crates/server/src/openapi.rs:28:10
//    |
// 28 | #[derive(OpenApi)]
//    |          ^^^^^^^
//
// so we manually do it ourselves here
impl Modify for IncludeErrorProneDatatypes {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = Into::<ComponentsBuilder>::into(openapi.components.take().unwrap())
            .schema_from::<charted_types::DateTime>()
            .schemas_from_iter({
                let mut schemas = Vec::new();
                <charted_types::DateTime as utoipa::ToSchema>::schemas(&mut schemas);

                schemas
            })
            .schema_from::<charted_core::serde::Duration>()
            .schemas_from_iter({
                let mut schemas = Vec::new();
                <charted_core::serde::Duration as utoipa::ToSchema>::schemas(&mut schemas);

                schemas
            })
            .schema_from::<charted_types::Ulid>()
            .schemas_from_iter({
                let mut schemas = Vec::new();
                <charted_types::Ulid as utoipa::ToSchema>::schemas(&mut schemas);

                schemas
            })
            .schema_from::<crate::openapi::Url>()
            .schemas_from_iter({
                let mut schemas = Vec::new();
                <crate::openapi::Url as utoipa::ToSchema>::schemas(&mut schemas);

                schemas
            })
            .build();

        openapi.components = Some(components);
    }
}

pub struct SecuritySchemes;

impl Modify for SecuritySchemes {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = Into::<ComponentsBuilder>::into(openapi.components.take().unwrap())
            .security_scheme(
                "ApiKey",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("ApiKey"))),
            )
            .security_scheme(
                "Bearer",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .description(Some(BEARER_SCHEME_DESCRIPTION))
                        .build(),
                ),
            )
            .security_scheme(
                "Basic",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Basic)
                        .description(Some(BASIC_SCHEME_DESCRIPTION))
                        .build(),
                ),
            )
            .build();

        openapi.components = Some(components);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utoipa::OpenApi;

    #[test]
    fn update_paths_to_include_default_api_version() {
        // a dummy route that exists
        #[utoipa::path(get, path = "/v1/weow")]
        #[allow(dead_code)]
        fn dummy_route() {}

        #[utoipa::path(get, path = "/v1")]
        #[allow(dead_code)]
        fn other_dummy_route() {}

        #[derive(OpenApi)]
        #[openapi(paths(dummy_route, other_dummy_route), modifiers(&UpdatePathsToIncludeDefaultVersion))]
        struct Document;

        let openapi = Document::openapi();
        let paths = openapi.paths.paths.keys().collect::<Vec<_>>();

        assert_eq!(&paths, &["/", "/v1", "/v1/weow", "/weow"]);
    }
}
