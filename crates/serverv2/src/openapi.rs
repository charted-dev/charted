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

mod addons;
mod types;

pub use types::{
    ApiErrorResponse, ApiKeyResponse, EmptyApiResponse, OrganizationResponse, RepositoryResponse, UserResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "charted-server",
        description = "🐻‍❄️📦 Free, open-source way to distribute Helm charts across the world",
        version = charted_core::VERSION,
        terms_of_service = "https://charts.noelware.org/legal/tos",
        license(identifier = "Apache-2.0", name = "Apache 2.0 License"),
        contact(
            name = "Noelware, LLC.",
            email = "team@noelware.org",
            url = "https://noelware.org"
        )
    ),
    components(
        responses(
            ApiErrorResponse,
            ApiKeyResponse,
            EmptyApiResponse,
            OrganizationResponse,
            RepositoryResponse,
            UserResponse
        )
    ),
    tags(
        (
            name = "Users",
            description = "Endpoints that create, modify, delete, or fetch user metadata"
        ),
        (
            name = "Users/Avatars",
            description = "Endpoints that can create, modify, delete, and fetch user avatars"
        ),
        (
            name = "Users/Sessions",
            description = "Endpoints that allow to login as a user and get an access token."
        ),
        (
            name = "API Keys",
            description = "Endpoints that allow authenticating users with a secret key that is trusted by the server."
        ),
        (
            name = "Repositories",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository metadata"
        ),
        (
            name = "Repository/Releases",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository releases"
        ),
        (
            name = "Repository/Members",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository members"
        ),
        (
            name = "Organizations",
            description = "Endpoints that create, modify, delete, or fetch organization metadata"
        ),
        (
            name = "Organization/Members",
            description = "Endpoints that create, modify, delete, or fetch organization members"
        ),
    ),
    servers(
        (
            url = "https://charts.noelware.org/api/v{version}",
            description = "Production Server",
            variables(
                ("version" = (
                    default = "1",
                    description = "Revision of the HTTP specification",
                    enum_values("1")
                ))
            )
        )
    ),
    external_docs(url = "https://charts.noelware.org/docs/server/latest")
)]
pub struct Document;

impl Document {
    /// Returns a prettified JSON document of the OpenAPI document.
    pub fn to_json_pretty() -> serde_json::Result<String> {
        serde_json::to_string_pretty(&Document::openapi())
    }
}

/*
#[derive(OpenApi)]
#[openapi(
    modifiers(
        &UpdatePathsToIncludeDefaultVersion,
        &IncludeErrorProneDatatypes,
        &SecuritySchemes
    ),
    components(
        schemas(
            //                          request bodies                          \\
            charted_types::payloads::CreateRepositoryReleasePayload,
            charted_types::payloads::PatchRepositoryReleasePayload,
            charted_types::payloads::CreateOrganizationPayload,
            charted_types::payloads::PatchOrganizationPayload,
            charted_types::payloads::CreateRepositoryPayload,
            charted_types::payloads::PatchRepositoryPayload,
            charted_types::payloads::CreateApiKeyPayload,
            charted_types::payloads::PatchApiKeyPayload,
            charted_types::payloads::CreateUserPayload,
            charted_types::payloads::PatchUserPayload,

            //                                scopes                            \\
            charted_core::bitflags::ApiKeyScope,

            //                              helm types                          \\
            charted_helm_types::StringOrImportValue,
            charted_helm_types::ChartSpecVersion,
            charted_helm_types::ChartMaintainer,
            charted_helm_types::ChartDependency,
            charted_helm_types::ChartIndexSpec,
            charted_helm_types::ImportValue,
            charted_helm_types::ChartIndex,
            charted_helm_types::ChartType,
            charted_helm_types::Chart,

            //                                entities                          \\
            charted_types::RepositoryRelease,
            charted_types::RepositoryMember,
            charted_types::Repository,

            charted_types::OrganizationMember,
            charted_types::Organization,

            charted_types::UserConnections,
            charted_types::Session,
            charted_types::ApiKey,
            charted_types::User,

            charted_core::api::ErrorCode,
            charted_core::api::Error,
            charted_core::Distribution,
            charted_core::BuildInfo,

            charted_types::NameOrUlid,
            charted_types::name::Name,
            charted_types::VersionReq,
            charted_types::Version,

            charted_types::payloads::UserLoginPayload,

            crate::routing::v1::main::Main,
            crate::routing::v1::features::Features,

            crate::pagination::PaginationRequest
        ),
        responses(
            ListApiResponse<ApiKey>,
            ListApiResponse<Repository>,
            ListApiResponse<RepositoryMember>,
            ListApiResponse<Organization>,
            ListApiResponse<OrganizationMember>,

            ApiResponse<Entrypoint>,
            ApiResponse<Features>,
            ApiResponse<Session>,
            ApiResponse<ApiKey>,
            ApiResponse<User>,
            ApiResponse<Main>,
            ApiResponse<Url>,
        )
    ),
    paths(
        crate::routing::v1::repository::releases::fetch_releases,
        crate::routing::v1::repository::releases::get_single_release,
        crate::routing::v1::repository::releases::get_single_release_provenance,
        crate::routing::v1::repository::releases::upload_release_tarball,
        crate::routing::v1::repository::releases::upload_release_provenance_tarball,
        crate::routing::v1::repository::releases::create,
        crate::routing::v1::repository::releases::patch,
        crate::routing::v1::repository::releases::delete,

        crate::routing::v1::repository::fetch,
        crate::routing::v1::repository::main,

        crate::routing::v1::user::sessions::login,
        crate::routing::v1::user::sessions::logout,
        crate::routing::v1::user::sessions::fetch,
        crate::routing::v1::user::sessions::refresh_session,

        crate::routing::v1::user::avatars::get_self_user_avatar_by_hash,
        crate::routing::v1::user::avatars::get_user_avatar_by_hash,
        crate::routing::v1::user::avatars::get_self_user_avatar,
        crate::routing::v1::user::avatars::upload_user_avatar,
        crate::routing::v1::user::avatars::get_user_avatar,

        crate::routing::v1::user::apikeys::delete,
        crate::routing::v1::user::apikeys::create,
        crate::routing::v1::user::apikeys::patch,
        crate::routing::v1::user::apikeys::fetch,
        crate::routing::v1::user::apikeys::list,

        crate::routing::v1::user::repositories::list_self_user_repositories,
        crate::routing::v1::user::repositories::list_user_repositories,
        crate::routing::v1::user::repositories::create_user_repository,

        crate::routing::v1::user::get_self,
        crate::routing::v1::user::create,
        crate::routing::v1::user::delete,
        crate::routing::v1::user::patch,
        crate::routing::v1::user::fetch,
        crate::routing::v1::user::main,

        crate::routing::v1::features::features,
        crate::routing::v1::healthz::healthz,
        crate::routing::v1::index::fetch,
        crate::routing::v1::main::main,
    ),
    tags(
        (
            name = "Main",
            description = "Represents all the main routes that don't tie to any entity"
        ),
        (
            name = "Users",
            description = "Endpoints that create, modify, delete, or fetch user metadata"
        ),
        (
            name = "Users/Avatars",
            description = "Endpoints that can create, modify, delete, and fetch user avatars"
        ),
        (
            name = "Users/Sessions",
            description = "Endpoints that allow to login as a user and get an access token."
        ),
        (
            name = "API Keys",
            description = "Endpoints that allow authenticating users with a secret key that is trusted by the server."
        ),
        (
            name = "Repositories",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository metadata"
        ),
        (
            name = "Repository/Releases",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository releases"
        ),
        (
            name = "Repository/Members",
            description = "Endpoints that create, modify, delete, or fetch user/organization repository members"
        ),
        (
            name = "Organizations",
            description = "Endpoints that create, modify, delete, or fetch organization metadata"
        ),
        (
            name = "Organization/Members",
            description = "Endpoints that create, modify, delete, or fetch organization members"
        ),
    ),
    servers(
        (
            url = "https://charts.noelware.org/api/v{version}",
            description = "Production Server",
            variables(
                ("version" = (
                    default = "1",
                    description = "Revision of the HTTP specification",
                    enum_values("1")
                ))
            )
        )
    ),
    external_docs(url = "https://charts.noelware.org/docs/server/latest")
)]
pub struct Document;

impl Document {
    pub fn to_json_pretty() -> serde_json::Result<String> {
        serde_json::to_string_pretty(&Document::openapi())
    }
}

/// Represents a generic empty API response, please do not use this in actual code,
/// it is only meant for utoipa for OpenAPI code generation.
pub struct EmptyApiResponse;

impl PartialSchema for EmptyApiResponse {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .property(
                "success",
                RefOr::T(Schema::Object({
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::Boolean))
                        .description(Some("Whether if this request was a success"))
                        .build()
                })),
            )
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for EmptyApiResponse {}

impl<'r> ToResponse<'r> for EmptyApiResponse {
    fn response() -> (Cow<'r, str>, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that doesn't contain any data")
            .content(
                "application/json",
                ContentBuilder::new().schema(Some(EmptyApiResponse::schema())).build(),
            )
            .build();

        (Cow::Borrowed("EmptyApiResponse"), RefOr::T(response))
    }
}

/// Represents a generic API error response object. Please do not use this in actual code,
/// it is only meant for OpenAPI code generation.
pub struct ApiErrorResponse;

impl PartialSchema for ApiErrorResponse {
    fn schema() -> RefOr<Schema> {
        let object = ObjectBuilder::new()
            .property(
                "success",
                RefOr::T(Schema::Object({
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::Boolean))
                        .description(Some("Whether if this request was a success or not (always false)"))
                        .default(Some(Value::Bool(false)))
                        .build()
                })),
            )
            .property(
                "errors",
                RefOr::T(Schema::Array({
                    ArrayBuilder::new()
                        .description(Some(
                            "List of errors that happened. This can be represented as a stacktrace",
                        ))
                        .items(RefOr::Ref(Ref::from_schema_name("Error")))
                        .build()
                })),
            )
            .build();

        RefOr::T(Schema::Object(object))
    }
}

impl ToSchema for ApiErrorResponse {}

impl<'r> ToResponse<'r> for ApiErrorResponse {
    fn response() -> (Cow<'r, str>, RefOr<Response>) {
        let response = ResponseBuilder::new()
            .description("API response that is returned during a error path")
            .content(
                "application/json",
                ContentBuilder::new().schema(Some(ApiErrorResponse::schema())).build(),
            )
            .build();

        (Cow::Borrowed("ApiErrorResponse"), RefOr::T(response))
    }
}

// TODO(@auguwu): once https://github.com/juhaku/utoipa/issues/1335 is fixed, move
// `ApiResponse`'s impl of ToResponse to `api::Response` and `ListApiResponse` to
// `charted_core`.

/// A [`Response`] type for
/// <code>[`api::Response`](charted_core::api::Response)\<T\></code> types.
pub struct ApiResponse<T: ?Sized>(PhantomData<T>);

impl<'r, T: ToSchema> ToResponse<'r> for ApiResponse<T> {
    fn response() -> (
        Cow<'r, str>,
        utoipa::openapi::RefOr<utoipa::openapi::response::Response>,
    ) {
        let name = T::name();
        let RefOr::T(Schema::Object(response_schema)) =
            <charted_core::api::Response<()> as PartialSchema>::schema()
        else {
            unreachable!()
        };

        let success = response_schema.properties.get("success").unwrap();
        let errors = response_schema.properties.get("errors").unwrap();
        let response = Response::builder()
            .description(format!("Response datatype that returns a `{name}` object"))
            .content(
                "application/json",
                Content::builder()
                    .schema(Some(RefOr::T(Schema::Object(
                        Object::builder()
                            .property("success", success.to_owned())
                            .required("success")
                            .property("data", RefOr::Ref(Ref::from_schema_name(T::name())))
                            .property("errors", errors.to_owned())
                            .build(),
                    ))))
                    .build(),
            )
            .build();

        (Cow::Owned(format!("{}Response", name)), RefOr::T(response))
    }
}

/// A [`Response`] type for
/// <code>[`api::Response`](charted_core::api::Response)\<[`Vec`]\<T\>\></code> types.
#[derive(Debug, Clone, Copy)]
pub struct ListApiResponse<T>(PhantomData<T>);
impl<T> Default for ListApiResponse<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'r, T: ToSchema> ToResponse<'r> for ListApiResponse<T> {
    fn response() -> (Cow<'r, str>, RefOr<Response>) {
        let name = T::name();
        let RefOr::T(Schema::Object(response_schema)) =
            <charted_core::api::Response<()> as PartialSchema>::schema()
        else {
            unreachable!()
        };

        let success = response_schema.properties.get("success").unwrap();
        let errors = response_schema.properties.get("errors").unwrap();
        let response = Response::builder()
            .description(format!("Response datatype for a list of `{name}`"))
            .content(
                "application/json",
                Content::builder()
                    .schema(Some(RefOr::T(Schema::Object(
                        Object::builder()
                            .property("success", success.to_owned())
                            .required("success")
                            .property(
                                "data",
                                RefOr::T(Schema::Array(Array::new(RefOr::Ref(Ref::from_schema_name(T::name()))))),
                            )
                            .property("errors", errors.to_owned())
                            .build(),
                    ))))
                    .build(),
            )
            .build();

        (Cow::Owned(format!("List{}Response", name)), RefOr::T(response))
    }
}
#[cfg(test)]
mod tests {
    use super::Document;
    use utoipa::OpenApi;

    /// A sanity check for all tests if all the references that are
    /// used are all correct and avaliable.
    #[test]
    fn sanity_check_if_all_references_are_correct() {}

    #[test]
    fn sanity_check_if_all_tags_are_correct() {
        let doc = Document::openapi();

        // we already know we have tags already -- we shouldn't have to add a
        // panic path here.
        let tags = unsafe { doc.tags.unwrap_unchecked() };

        for (path, item) in doc.paths.paths {
            for (method, op) in [
                ("get", item.get),
                ("put", item.put),
                ("post", item.post),
                ("patch", item.patch),
                ("delete", item.delete),
            ] {
                let Some(op) = op else {
                    continue;
                };

                if let Some(optags) = op.tags {
                    for tag in optags {
                        assert!(
                            tags.iter().any(|x| x.name == tag),
                            "operation {method} {path}: tag '{tag}' doesn't exist in openapi document"
                        );
                    }
                }
            }
        }
    }
}

*/

#[cfg(test)]
mod tests;
