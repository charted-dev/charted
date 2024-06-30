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

pub mod apikey;
pub(crate) mod cdn;
pub mod features;
pub mod heartbeat;
pub mod index;
pub mod info;
pub mod main;
pub(crate) mod metrics;
pub mod organization;
pub mod repository;
pub mod user;

use crate::ServerContext;
use axum::{routing, Router};
use charted_proc_macros::add_paths;
use tracing::error;
use utoipa::openapi::Paths;

pub fn create_router(ctx: &ServerContext) -> Router<ServerContext> {
    let mut router = Router::new()
        .route("/heartbeat", routing::get(heartbeat::HeartbeatRestController::run))
        .route("/metrics", routing::get(metrics::metrics))
        .route("/index", routing::get(index::GetChartIndexRestController::run))
        .route("/info", routing::get(info::InfoRestController::run))
        .route("/", routing::get(main::MainRestController::run));

    if ctx.config.cdn.enabled {
        let prefix = match ctx.config.cdn.prefix {
            Some(ref prefix) => {
                if !prefix.starts_with('/') {
                    error!(%prefix, "invalid cdn prefix, must be a valid path! opting to /cdn instead");
                    "/cdn".into()
                } else {
                    prefix.clone()
                }
            }

            None => "/cdn".into(),
        };

        let final_path = format!("{}/*file", prefix.trim_end_matches('/'));
        router = router.clone().route(&final_path, routing::get(cdn::cdn));
    }

    router
}

pub fn paths() -> Paths {
    add_paths!(
        // ~~~~~~~~~~~      MAIN     ~~~~~~~~~~~~~~~~~~~
        "/v1/index/{idOrName}" => index::GetChartIndexRestController::paths();
        "/v1/heartbeat" => heartbeat::HeartbeatRestController::paths();
        "/v1/features" => features::FeaturesRestController::paths();
        "/v1/info" => info::InfoRestController::paths();
        "/v1" => main::MainRestController::paths();
    )
}

/*
    /// [`Paths`] of all available [`APIVersion::V1`] endpoints.
    pub fn v1() -> Paths {
        add_paths! {
            // ~~~~~~~~~~~ API KEYS ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/apikeys/{idOrName}") => [];
            Document::format(APIVersion::V1, "/apikeys/all") => crate::server::routing::v1::apikey::ListAllApikeysRestController::paths();
            Document::format(APIVersion::V1, "/apikeys") => [
                crate::server::routing::v1::apikey::EntrypointRestController::paths()
            ];

            // ~~~~~~~~~~~ ORGANIZATIONS ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/organizations/{idOrName}/repositories") => crate::server::routing::v1::organization::repositories::ListOrgRepositoriesRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}/icon/{hash}") => crate::server::routing::v1::organization::icons::GetOrgIconByHashRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}/icon") => crate::server::routing::v1::organization::icons::GetCurrentOrgIconRestController::paths();
            Document::format(APIVersion::V1, "/organizations/{idOrName}") => crate::server::routing::v1::organization::GetOrgByIdOrNameRestController::paths();
            Document::format(APIVersion::V1, "/organizations") => crate::server::routing::v1::organization::EntrypointRestController::paths();

            // ~~~~~~~~~~~ REPOSITORIES ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/repositories/{id}/releases/{version}/provenance") => [
                // crate::server::routing::v1::repository::releases::PutReleaseProvenanceTarballRestController::paths(),
                crate::server::routing::v1::repository::releases::GetReleaseProvenanceFileRestController::paths(),
            ];

            Document::format(APIVersion::V1, "/repositories/{id}/releases/{version}/tarball") => [
                crate::server::routing::v1::repository::releases::GetReleaseTarballRestController::paths(),
                crate::server::routing::v1::repository::releases::PutReleaseTarballRestController::paths(),
            ];

            Document::format(APIVersion::V1, "/repositories/{id}/releases/{version}") => [
                crate::server::routing::v1::repository::releases::GetRepositoryReleaseByTagRestController::paths(),
                crate::server::routing::v1::repository::releases::PatchRepositoryReleaseRestController::paths(),
                crate::server::routing::v1::repository::releases::DeleteRepositoryReleaseRestController::paths()
            ];

            Document::format(APIVersion::V1, "/repositories/{id}/releases") => [
                crate::server::routing::v1::repository::releases::GetAllRepositoryReleasesRestController::paths(),
                crate::server::routing::v1::repository::releases::CreateRepositoryReleaseRestController::paths(),
            ];

            Document::format(APIVersion::V1, "/repositories/{owner}/{name}") => crate::server::routing::v1::repository::GetRepoByOwnerAndNameRestController::paths();
            Document::format(APIVersion::V1, "/repositories/{id}") => crate::server::routing::v1::repository::GetRepoByIdRestController::paths();
            Document::format(APIVersion::V1, "/repositories") => crate::server::routing::v1::repository::EntrypointRestController::paths();

            // ~~~~~~~~~~~     USERS     ~~~~~~~~~~~~~~~~~~~
            Document::format(APIVersion::V1, "/users/{idOrName}/avatar/{hash}") => crate::server::routing::v1::user::avatars::GetUserAvatarByHashRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}/repositories") => crate::server::routing::v1::user::repositories::ListUserRepositoriesRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}/avatar") => crate::server::routing::v1::user::avatars::GetCurrentUserAvatarRestController::paths();
            Document::format(APIVersion::V1, "/users/{idOrName}") => crate::server::routing::v1::user::GetUserRestController::paths();

            Document::format(APIVersion::V1, "/users/@me/avatar/{hash}") => crate::server::routing::v1::user::avatars::GetSelfUserAvatarByHashRestController::paths();
            Document::format(APIVersion::V1, "/users/@me/avatar") => [
                crate::server::routing::v1::user::avatars::GetSelfUserAvatarRestController::paths(),
                crate::server::routing::v1::user::avatars::UploadAvatarRestController::paths()
            ];

            Document::format(APIVersion::V1, "/users/@me/repositories") => crate::server::routing::v1::user::repositories::CreateUserRepositoryRestController::paths();
            Document::format(APIVersion::V1, "/users/@me") => crate::server::routing::v1::user::GetSelfRestController::paths();
            Document::format(APIVersion::V1, "/users") => [
                crate::server::routing::v1::user::DeleteSelfRestController::paths(),
                crate::server::routing::v1::user::CreateUserRestController::paths(),
                crate::server::routing::v1::user::PatchRestController::paths(),
                crate::server::routing::v1::user::MainRestController::paths()
            ];
        }
    }
*/
