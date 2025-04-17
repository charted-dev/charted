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
    Context,
    extract::{Path, Query},
    middleware::authn::{self, Options},
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
    routing,
};
use azalia::remi::core::Blob;
use charted_core::{api, clamp};
use charted_database::entities::{RepositoryReleaseEntity, repository::release};
use charted_types::{NameOrUlid, RepositoryRelease, Ulid, Version};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cmp;
use utoipa::{
    IntoParams, ToSchema,
    openapi::{
        Ref, RefOr, Required,
        path::{Parameter, ParameterIn},
    },
};

/// Union enumeration that can take either a [`Version`] (a repository release's tag)
/// or a [`Ulid`], which can be a repository release's unique identifier.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum VersionOrUlid {
    Version(Version),
    Ulid(Ulid),
}

impl IntoParams for VersionOrUlid {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        [Parameter::builder()
            .name("versionOrId")
            .required(Required::True)
            .parameter_in(parameter_in_provider().unwrap_or_default())
            .description(Some("A path parameter that can take either a **Version** representing the release tag or a **Ulid** which can represent a specific release by its unique identifier"))
            .schema(Some(RefOr::Ref(Ref::from_schema_name("VersionOrUlid"))))
            .build()]
        .to_vec()
    }
}

pub fn create_router(cx: &Context) -> Router<Context> {
    Router::new()
        .route(
            "/",
            routing::get(fetch_releases.layer(authn::new(cx.to_owned(), Options {
                allow_unauthorized: true,
                ..Default::default()
            }))),
        )
        .route(
            "/{versionOrId}",
            routing::get(get_single_release.layer(authn::new(cx.to_owned(), Options {
                allow_unauthorized: true,
                ..Default::default()
            }))),
        )
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
    State(cx): State<Context>,
    Path((owner, repo)): Path<(NameOrUlid, NameOrUlid)>,
    Query(PaginationRequest {
        per_page,
        order_by,
        page,
    }): Query<PaginationRequest>,
) -> api::Result<Vec<RepositoryRelease>> {
    let per_page = clamp(per_page, 10, 100).unwrap_or(10);
    let repository = super::fetch(State(cx.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    let paginator = RepositoryReleaseEntity::find()
        .filter(release::Column::Repository.eq(repository.id))
        .order_by(release::Column::Id, order_by.into_sea_orm())
        .paginate(&cx.pool, per_page as u64);

    let pages = paginator.num_pages().await.map_err(api::system_failure)?;
    let entries = paginator
        .fetch_page(cmp::min(0, page as u64))
        .await
        .map_err(api::system_failure)?
        .into_iter()
        .map(Into::<RepositoryRelease>::into)
        .collect::<Vec<_>>();

    let resource = cx
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
    State(cx): State<Context>,
    Path((owner, repo, version)): Path<(NameOrUlid, NameOrUlid, VersionOrUlid)>,
) -> api::Result<RepositoryRelease> {
    let repository = super::fetch(State(cx.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    match (match version {
        VersionOrUlid::Version(ref version) => RepositoryReleaseEntity::find()
            .filter(release::Column::Repository.eq(repository.id))
            .filter(release::Column::Tag.eq(version.clone())),

        VersionOrUlid::Ulid(id) => {
            RepositoryReleaseEntity::find_by_id(id).filter(release::Column::Repository.eq(repository.id))
        }
    })
    .one(&cx.pool)
    .await
    .map_err(api::system_failure)?
    .map(Into::<RepositoryRelease>::into)
    {
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

type StorageServiceRet = ([(header::HeaderName, HeaderValue); 2], azalia::remi::core::Bytes);

/// Retrieve a repository release's provenance file, if one was provided
/// with `helm charted upload --sign`.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}/provenance",
    operation_id = "getRepositoryReleaseProvenance",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid, QueryParams)
)]
pub async fn get_single_release_provenance(
    State(cx): State<Context>,
    Path((owner, repo, version)): Path<(NameOrUlid, NameOrUlid, VersionOrUlid)>,
    Query(QueryParams { prereleases }): Query<QueryParams>,
) -> Result<StorageServiceRet, api::Response> {
    let repository = super::fetch(State(cx.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    let Some(release) = (match version {
        VersionOrUlid::Version(ref version) => RepositoryReleaseEntity::find()
            .filter(release::Column::Repository.eq(repository.id))
            .filter(release::Column::Tag.eq(version.clone())),

        VersionOrUlid::Ulid(id) => {
            RepositoryReleaseEntity::find_by_id(id).filter(release::Column::Repository.eq(repository.id))
        }
    })
    .one(&cx.pool)
    .await
    .map_err(api::system_failure)?
    .map(Into::<RepositoryRelease>::into) else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository release with version or ulid was not found",
                json!({"versionOrUlid":version}),
            ),
        ));
    };

    let Some(Blob::File(file)) = charted_helm_charts::get_chart_provenance(
        &cx.storage,
        repository.owner,
        repository.id,
        release.tag.to_string(),
        prereleases,
    )
    .await
    .map_err(api::system_failure_from_report)?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository release doesn't contain a provenance signature",
            ),
        ));
    };

    let headers = [
        (
            header::CONTENT_TYPE,
            HeaderValue::from_str(&azalia::remi::fs::default_resolver(file.data.as_ref())).unwrap(),
        ),
        (header::CONTENT_LENGTH, HeaderValue::from(file.size)),
    ];

    Ok((headers, file.data))
}

/// Returns a tarball of the release if the server has ever received it.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}/tarball",
    operation_id = "getRepositoryReleaseChartTarball",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn get_chart_tarball(
    State(cx): State<Context>,
    Path((owner, repo, version)): Path<(NameOrUlid, NameOrUlid, VersionOrUlid)>,
    Query(QueryParams { prereleases }): Query<QueryParams>,
) -> Result<StorageServiceRet, api::Response> {
    let repository = super::fetch(State(cx.clone()), Path((owner.clone(), repo.clone())))
        .await?
        .data
        .unwrap();

    let Some(release) = (match version {
        VersionOrUlid::Version(ref version) => RepositoryReleaseEntity::find()
            .filter(release::Column::Repository.eq(repository.id))
            .filter(release::Column::Tag.eq(version.clone())),

        VersionOrUlid::Ulid(id) => {
            RepositoryReleaseEntity::find_by_id(id).filter(release::Column::Repository.eq(repository.id))
        }
    })
    .one(&cx.pool)
    .await
    .map_err(api::system_failure)?
    .map(Into::<RepositoryRelease>::into) else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository release with version or ulid was not found",
                json!({"versionOrUlid":version}),
            ),
        ));
    };

    let Some(Blob::File(file)) = charted_helm_charts::get_chart(
        &cx.storage,
        repository.owner,
        repository.id,
        release.tag.to_string(),
        prereleases,
    )
    .await
    .map_err(api::system_failure_from_report)?
    else {
        return Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "repository release doesn't contain a tarball to be used with `helm install`",
            ),
        ));
    };

    let headers = [
        (
            header::CONTENT_TYPE,
            HeaderValue::from_str(&azalia::remi::fs::default_resolver(file.data.as_ref())).unwrap(),
        ),
        (header::CONTENT_LENGTH, HeaderValue::from(file.size)),
    ];

    Ok((headers, file.data))
}

/// Publish a chart that is linked with this release.
///
/// This method will fail if **PUT /repositories/{owner}/{repo}/releases** was
/// not called first.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    post,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}/tarball",
    operation_id = "publishRepositoryReleaseChartTarball",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn upload_release_tarball() {}

/// Publish a chart's provenance file that is linked with this release.
///
/// This method will fail if **PUT /repositories/{owner}/{repo}/releases** was
/// not called first.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    post,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}/provenance",
    operation_id = "pushRepositoryReleaseProvenanceFile",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn upload_release_provenance_tarball() {}

/// Creates a repository release.
///
/// This will not upload a chart tarball, use the **POST
/// /repositories/{owner}/{repo}/releases/{version}/tarball** endpoint.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    put,

    path = "/v1/repositories/{owner}/{repo}/releases",
    operation_id = "createRepositoryRelease",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP)
)]
pub async fn create() {}

/// Patch a repository release's metadata.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    patch,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}",
    operation_id = "patchRepositoryRelease",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn patch() {}

/// Deletes this repository release.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    delete,

    path = "/v1/repositories/{owner}/{repo}/releases/{versionOrId}",
    operation_id = "deleteRepositoryRelease",
    tags = ["Repositories", "Repository/Releases"],
    params(OwnerRepoP, VersionOrUlid)
)]
pub async fn delete() {}
