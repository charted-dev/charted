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

use crate::{
    Env,
    extract::{Path, Query},
    middleware::authn::{Factory, Options},
    ops::db,
    pagination::PaginationRequest,
    routing::v1::repository::OwnerRepoP,
    util::{self, BuildLinkHeaderOpts},
};
use axum::{
    Router,
    extract::State,
    handler::Handler,
    http::{
        StatusCode,
        header::{self, HeaderValue},
    },
    response::IntoResponse,
    routing,
};
use charted_core::{
    api,
    bitflags::{ApiKeyScope, ApiKeyScopes},
    clamp,
};
use charted_database::entities::{RepositoryReleaseEntity, repository::release};
use charted_datastore::fs;
use charted_helm_charts::DataStoreExt;
use charted_types::{NameOrUlid, QueryableVersion, RepositoryRelease, Ulid, VersionOrUlid};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::Deserialize;
use serde_json::json;
use std::cmp;
use utoipa::IntoParams;

pub fn create_router(env: &Env) -> Router<Env> {
    let router = Router::new().route(
        "/",
        routing::get(fetch_releases.layer(env.authn(Options {
            allow_unauthorized: true,
            scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),

            ..Default::default()
        }))),
    );

    let id = Router::new()
        .route(
            "/tarball",
            routing::get(get_single_release_tarball.layer(env.authn(Options {
                allow_unauthorized: true,
                scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),

                ..Default::default()
            }))),
        )
        .route(
            "/provenance",
            routing::get(get_single_release_provenance.layer(env.authn(Options {
                allow_unauthorized: true,
                scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),

                ..Default::default()
            }))),
        );

    let version_or_ulid = Router::new().route(
        "/",
        routing::get(get_single_release.layer(env.authn(Options {
            allow_unauthorized: true,
            scopes: ApiKeyScopes::new(ApiKeyScope::RepoAccess.into()),

            ..Default::default()
        }))),
    );

    router
        .nest("/{id}/{version}", id)
        .nest("/{versionOrId}", version_or_ulid)
}

/// Lists all the avaliable releases for this repository.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases",
    operation_id = "listRepositoryReleases",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, PaginationRequest)
)]
pub async fn fetch_releases(
    State(env): State<Env>,
    Path((owner, repo)): Path<(NameOrUlid, NameOrUlid)>,
    Query(PaginationRequest {
        per_page,
        order_by,
        page,
    }): Query<PaginationRequest>,
) -> api::Result<Vec<RepositoryRelease>> {
    let per_page = clamp(per_page, 10, 100).unwrap_or(10);
    let repository = super::fetch(State(env.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    let paginator = RepositoryReleaseEntity::find()
        .filter(release::Column::Repository.eq(repository.id))
        .order_by(release::Column::Id, order_by.into_sea_orm())
        .paginate(&env.db, per_page as u64);

    let pages = paginator.num_pages().await.map_err(api::system_failure)?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .map_err(api::system_failure)?
        .into_iter()
        .map(Into::<RepositoryRelease>::into)
        .collect::<Vec<_>>();

    let resource = env
        .config
        .base_url
        .unwrap()
        .join(&format!("/v1/repositories/{owner}/{repo}/releases"))
        .unwrap();

    let mut link_hdr = String::new();
    util::build_link_header(&mut link_hdr, BuildLinkHeaderOpts {
        entries: entries.len(),
        current: page,
        per_page,
        max_pages: pages,
        resource,
    })
    .map_err(api::system_failure)?;

    let mut response = api::ok(StatusCode::OK, entries);
    if !link_hdr.is_empty() {
        response = response.with_header(header::LINK, HeaderValue::from_bytes(link_hdr.as_bytes()).unwrap());
    }

    Ok(response)
}

/// Retrieve a single repository release.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}",
    operation_id = "getRepositoryRelease",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn get_single_release(
    State(env): State<Env>,
    Path((owner, repo, version)): Path<(NameOrUlid, NameOrUlid, VersionOrUlid)>,
) -> api::Result<RepositoryRelease> {
    let repository = super::fetch(State(env.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    match db::repository::release::get(&env.db, &repository, version.clone()).await? {
        Some(release) => Ok(api::ok(StatusCode::OK, release)),
        None => Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository release with version or ulid was not found",
                json!({"versionOrUlid":version}),
            ),
        )),
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryParams {
    /// whether if querying for pre-releases is useful.
    pub prereleases: bool,
}

/// Retrieve a repository release's provenance file, if one was provided
/// with `helm charted upload --sign`.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases/{id}/{version}/provenance",
    operation_id = "getRepositoryReleaseProvenance",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, Ulid, QueryableVersion, QueryParams)
)]
pub async fn get_single_release_provenance(
    State(env): State<Env>,
    Path((owner, repo, id, version)): Path<(NameOrUlid, NameOrUlid, Ulid, QueryableVersion)>,
    Query(QueryParams { prereleases }): Query<QueryParams>,
) -> Result<impl IntoResponse, api::Response> {
    let repository = super::fetch(State(env.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    match db::repository::release::get(&env.db, &repository, VersionOrUlid::Ulid(id)).await? {
        Some(_) => {}
        None => {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository release with id was not found",
                    json!({"id":id}),
                ),
            ));
        }
    };

    let ns = env.ds.owner_repo(repository.owner, repository.id);
    let Some(chart_prov) = ns
        .get_chart_provenance(version, prereleases)
        .await
        .map_err(api::system_failure_from_report)?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "chart provenance for release was not found",
            ),
        ));
    };

    let ct =
        HeaderValue::from_str(&fs::default_resolver(chart_prov.data.as_ref())).map_err(api::system_failure)?;

    let headers = [(header::CONTENT_TYPE, ct), (header::CONTENT_LENGTH, HeaderValue::from(chart_prov.size))];
    Ok((headers, chart_prov.data))
}

/// Returns a tarball of the release if the server has ever received it.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases/{id}/{version}/tarball",
    operation_id = "getRepositoryReleaseChartTarball",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, Ulid, QueryableVersion, QueryParams)
)]
pub async fn get_single_release_tarball(
    State(env): State<Env>,
    Path((owner, repo, id, version)): Path<(NameOrUlid, NameOrUlid, Ulid, QueryableVersion)>,
    Query(QueryParams { prereleases }): Query<QueryParams>,
) -> Result<impl IntoResponse, api::Response> {
    let repository = super::fetch(State(env.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    match db::repository::release::get(&env.db, &repository, VersionOrUlid::Ulid(id)).await? {
        Some(_) => {}
        None => {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "repository release with id was not found",
                    json!({"id":id}),
                ),
            ));
        }
    };

    let ns = env.ds.owner_repo(repository.owner, repository.id);
    let Some(chart_prov) = ns
        .get_chart(version, prereleases)
        .await
        .map_err(api::system_failure_from_report)?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "chart provenance for release was not found",
            ),
        ));
    };

    let ct =
        HeaderValue::from_str(&fs::default_resolver(chart_prov.data.as_ref())).map_err(api::system_failure)?;

    let headers = [(header::CONTENT_TYPE, ct), (header::CONTENT_LENGTH, HeaderValue::from(chart_prov.size))];
    Ok((headers, chart_prov.data))
}
