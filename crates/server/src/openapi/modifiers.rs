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
use std::collections::{BTreeMap, HashMap, HashSet};
use utoipa::{
    openapi::{
        schema::ArrayItems,
        security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
        ArrayBuilder, ComponentsBuilder, OpenApi, Ref, RefOr, Schema,
    },
    Modify,
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
//    = note: this error originates in the derive macro `OpenApi` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// error: expected one of `.`, `;`, `?`, `else`, or an operator, found `)`
//   --> crates/server/src/openapi.rs:28:10
//    |
// 28 | #[derive(OpenApi)]
//    |          ^^^^^^^ expected one of `.`, `;`, `?`, `else`, or an operator
//    |
//    = note: this error originates in the derive macro `OpenApi` (in Nightly builds, run with -Z macro-backtrace for more info)
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

/// [Modifier][Modify] that replaces `Response_<type>` as `<type>Response`.
pub struct ResponseModifiers;

// While the implementation can be cleaned and optimized, for now it works.
impl Modify for ResponseModifiers {
    fn modify(&self, openapi: &mut OpenApi) {
        let mut components = openapi.components.take().unwrap();

        // 1. First, we need to update all `Response_<>` with `<>Response` and update the
        //    `data` field to be the type that we want
        let mut scheduled_to_be_removed = HashSet::new();
        let mut scheduled_to_be_created = HashMap::new();
        let schemas = components.schemas.clone();

        for (key, schema) in schemas
            .iter()
            .filter_map(|(key, schema)| key.starts_with("Response_").then_some((key, schema)))
        {
            // First, we need to schedule the deletion of `key` since we will
            // no longer be using it
            scheduled_to_be_removed.insert(key.as_str());

            // Update the schema's description
            //
            // We only expect `"type": "object"` as response types are only objects
            // and cannot be anything else
            let RefOr::T(Schema::Object(mut object)) = schema.clone() else {
                unreachable!();
            };

            let (_, mut ty) = key.split_once('_').unwrap();
            if ty.ends_with("Response") {
                ty = ty.trim_end_matches("Response");
            }

            object.description = Some(format!("Response datatype for the `{ty}` type"));
            assert!(object.properties.remove("data").is_some());

            object
                .properties
                .insert("data".into(), RefOr::Ref(Ref::from_schema_name(ty)));

            scheduled_to_be_created.insert(format!("{ty}Response"), RefOr::T(Schema::Object(object)));
        }

        scheduled_to_be_removed
            .iter()
            .map(|x| components.schemas.remove(*x))
            .for_each(drop);

        scheduled_to_be_created
            .iter()
            .map(|(key, value)| components.schemas.insert(key.to_owned(), value.clone()))
            .for_each(drop);

        // 2. Now, we need to go to every path and check if there is a ref
        //    with the `Response_<>` suffix
        {
            for item in openapi.paths.paths.values_mut() {
                macro_rules! do_update {
                    ($kind:ident as $op:expr) => {
                        if let Some(ref mut op) = $op {
                            for resp in op
                                .responses
                                .responses
                                .values_mut()
                                .filter_map(|resp| match resp {
                                    RefOr::T(resp) => Some(resp),
                                    _ => None,
                                })
                            {
                                for content in resp.content.values_mut() {
                                    if let Some(RefOr::Ref(ref_)) = content.schema.as_ref() {
                                        if ref_.ref_location.contains("Response_") {
                                            let reference = ref_.ref_location.split("/").last().unwrap();

                                            let (_, ty) = reference.split_once('_').unwrap();

                                            // if we get Response_Vec_ApiKey, then we will transform
                                            // it into `ApiResponse<Vec<ApiKey>>` by
                                            let schema = if ty.starts_with("Vec_") {
                                                let (_, inner) = ty.split_once('_').unwrap();
                                                assert!(!inner.contains("_"));

                                                RefOr::T(Schema::Array(
                                                    ArrayBuilder::new()
                                                        .items(ArrayItems::RefOrSchema(Box::new(RefOr::Ref(
                                                            Ref::from_schema_name(inner),
                                                        ))))
                                                        .build(),
                                                ))
                                            } else {
                                                assert!(!ty.contains("_"));
                                                RefOr::Ref(Ref::from_schema_name(format!("{ty}Response")))
                                            };

                                            content.schema = Some(schema);
                                        }
                                    }
                                }
                            }

                            item.$kind = ($op).clone();
                        }
                    };
                }

                do_update!(get as item.get);
                do_update!(put as item.put);
                do_update!(head as item.head);
                do_update!(post as item.post);
                do_update!(patch as item.patch);
                do_update!(trace as item.trace);
                do_update!(delete as item.delete);
                do_update!(delete as item.delete);
                do_update!(options as item.options);
            }
        }

        openapi.components = Some(components);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use charted_core::api;
    use charted_types::User;
    use utoipa::{openapi::HttpMethod, OpenApi};

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

    // This test combats using `Response_<>` as `<>Response`, where `<>` is
    // the type that is registered as a schema.
    #[test]
    fn update_response_types() {
        #[utoipa::path(get, path = "/", responses((status = 200, body = api::Response<User>)))]
        #[allow(unused)]
        fn test_path() {}

        #[derive(OpenApi)]
        #[openapi(paths(test_path), modifiers(&ResponseModifiers))]
        struct Document;

        let openapi = Document::openapi();

        // Check that `UserResponse` has the modified description and reference
        {
            let components = openapi.components.unwrap();
            let RefOr::T(Schema::Object(object)) = components.schemas.get("UserResponse").unwrap() else {
                unreachable!();
            };

            assert_eq!(
                object.description,
                Some("Response datatype for the `User` type".to_string())
            );

            let Some(RefOr::Ref(ref_)) = object.properties.get("data") else {
                unreachable!();
            };

            assert_eq!(ref_.ref_location, "#/components/schemas/User");
        }

        // Check if `GET /` has the updated response type
        {
            let paths = openapi.paths.clone();
            let Some(path) = paths.get_path_operation("/", HttpMethod::Get) else {
                unreachable!();
            };

            let Some(RefOr::T(resp)) = path.responses.responses.get("200") else {
                unreachable!();
            };

            let Some(content) = resp.content.get("application/json") else {
                unreachable!();
            };

            let Some(RefOr::Ref(ref ref_)) = content.schema else {
                unreachable!()
            };

            assert_eq!(ref_.ref_location, "#/components/schemas/UserResponse");
        }
    }
}
