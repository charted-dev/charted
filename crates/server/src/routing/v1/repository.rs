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

use crate::{
    Env, OwnerExt,
    ext::ResultExt,
    extract::Path,
    middleware::authn::{Factory, Options},
    mk_into_responses,
    openapi::RepositoryResponse,
    ops::db,
    routing::v1::Entrypoint,
};
use axum::{Router, extract::State, handler::Handler, http::StatusCode, routing};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
};
use charted_database::entities::repository;
use charted_types::{NameOrUlid, Owner, Repository};
use sea_orm::{ColumnTrait, QueryFilter};
use serde_json::json;
use utoipa::{
    IntoParams,
    openapi::path::{Parameter, ParameterIn},
};

pub fn create_router(env: &Env) -> Router<Env> {
    Router::new()
        .route("/", routing::get(main))
        .route(
            "/{owner}/{repo}",
            routing::get(fetch.layer(env.authn(Options {
                allow_unauthorized: true,
                scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),
                require_refresh_token: false,
            }))),
        )
        .nest("/releases/{owner}/{repo}", releases::create_router(env))
}

/// Entrypoint handler to the Repositories API.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/v1/repositories",
    operation_id = "repositories",
    tag = "Repositories",
    responses(Entrypoint)
)]
pub async fn main() -> api::Response<Entrypoint> {
    api::ok(StatusCode::OK, Entrypoint::new("Repositories"))
}

struct FetchRepoR;
mk_into_responses!(for FetchRepoR {
    "200" => [ref(RepositoryResponse)];
    "400" => [error(description("invalid id or name"))];
    "404" => [error(description("repository not found"))];
});

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
#[utoipa::path(
    get,
    path = "/v1/repositories/{owner}/{repo}",
    tag = "Users",
    operation_id = "getRepositoryByIdOrName",
    responses(FetchRepoR),
    params(OwnerRepoP)
)]
pub async fn fetch(
    State(env): State<Env>,
    Path((owner, repo)): Path<(NameOrUlid, NameOrUlid)>,
) -> api::Result<Repository> {
    let Some(owner) = Owner::query_by_id_or_name(&env, owner.clone())
        .await
        .into_system_failure()?
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

    match db::repository::get_with_additional_bounds(&env.db, repo.clone(), |query| {
        query
            .filter(repository::Column::Owner.eq(owner.id()))
            .filter(repository::Column::Private.eq(false))
    })
    .await?
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
    }
}
