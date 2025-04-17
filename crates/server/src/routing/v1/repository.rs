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

pub mod releases;

use super::Entrypoint;
use crate::{
    Context,
    ext::OwnerExt,
    extract::Path,
    extract_refor_t,
    middleware::authn::{self, Options},
    modify_property,
    openapi::ApiErrorResponse,
    routing::v1::EntrypointResponse,
};
use axum::{Router, extract::State, handler::Handler, http::StatusCode, routing};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
};
use charted_database::entities::{RepositoryEntity, repository};
use charted_types::{NameOrUlid, Owner, Repository};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::json;
use std::collections::BTreeMap;
use utoipa::{
    IntoParams, IntoResponses, ToResponse,
    openapi::{
        Ref, RefOr, Response,
        path::{Parameter, ParameterIn},
    },
};

pub fn create_router(cx: &Context) -> Router<Context> {
    Router::new()
        .route("/", routing::get(main))
        .route(
            "/{owner}/{repo}",
            routing::get(fetch.layer(authn::new(cx.to_owned(), Options {
                allow_unauthorized: true,
                scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),
                require_refresh_token: false,
            }))),
        )
        .nest("/{owner}/{repo}/releases", releases::create_router(cx))
}

/// Entrypoint handler to the Repositories API.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/v1/repositories",
    operation_id = "repositories",
    tag = "Repositories",
    responses(EntrypointResponse)
)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Repositories"))
}

struct FetchRepoR;
impl IntoResponses for FetchRepoR {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap! {
            "200" => Ref::from_response_name("RepositoryResponse"),
            "400" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Invalid ID or name specified"));

                response
            },

            "404" => {
                let mut response = extract_refor_t!(ApiErrorResponse::response().1);
                modify_property!(response.description("Repository not found"));

                response
            }
        }
    }
}

pub struct OwnerRepoP;
impl IntoParams for OwnerRepoP {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        vec![
            Parameter::builder()
                .name("owner")
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .description(Some("Queries the resource owned by this repository"))
                .schema(Some(utoipa::openapi::RefOr::Ref(
                    utoipa::openapi::Ref::from_schema_name("NameOrUlid"),
                )))
                .build(),
            Parameter::builder()
                .name("repo")
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .description(Some("Queries this resource by the entity's ID or name."))
                .schema(Some(utoipa::openapi::RefOr::Ref(
                    utoipa::openapi::Ref::from_schema_name("NameOrUlid"),
                )))
                .build(),
        ]
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
#[instrument(name = "charted.server.ops.fetch[repository]", skip_all, fields(%owner, %repo))]
#[utoipa::path(
    get,
    path = "/v1/repositories/{owner}/{repo}",
    tag = "Users",
    operation_id = "getRepositoryByIdOrName",
    responses(FetchRepoR),
    params(OwnerRepoP)
)]
pub async fn fetch(
    State(cx): State<Context>,
    Path((owner, repo)): Path<(NameOrUlid, NameOrUlid)>,
) -> api::Result<Repository> {
    let Some(owner) = Owner::query_by_id_or_name(&cx, owner.clone())
        .await
        .map_err(api::system_failure)?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "user or organization by either id or name was not found",
                json!({"idOrName":owner}),
            ),
        ));
    };

    match repo {
        NameOrUlid::Name(ref name) => match RepositoryEntity::find()
            .filter(repository::Column::Name.eq(name.clone()))
            .filter(repository::Column::Owner.eq(owner.id()))
            .filter(repository::Column::Private.eq(false))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::<Repository>::into)
        {
            Some(repo) => Ok(api::ok(StatusCode::OK, repo)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository with id or name was not found",
                    json!({"idOrName":repo}),
                ),
            )),
        },

        NameOrUlid::Ulid(id) => match RepositoryEntity::find_by_id(id)
            .filter(repository::Column::Owner.eq(owner.id()))
            .filter(repository::Column::Private.eq(false))
            .one(&cx.pool)
            .await
            .map_err(api::system_failure)?
            .map(Into::into)
        {
            Some(repo) => Ok(api::ok(StatusCode::OK, repo)),
            None => Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository with id or name was not found",
                    json!({"idOrName":repo}),
                ),
            )),
        },
    }
}
