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

use crate::ext::ResultExt;
use charted_core::api;
use charted_database::entities::{RepositoryReleaseEntity, repository::release};
use charted_types::{Repository, RepositoryRelease, VersionOrUlid};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, Select};

#[instrument(name = "charted.server.ops.getRepositoryRelease", skip_all, fields(versionOrId = %version_or_ulid, repository.id = %repo.id, repository.name = %repo.name))]
pub async fn get(
    db: &DbConn,
    repo: &Repository,
    version_or_ulid: VersionOrUlid,
) -> Result<Option<RepositoryRelease>, api::Response> {
    match version_or_ulid {
        VersionOrUlid::Ulid(id) => RepositoryReleaseEntity::find_by_id(id)
            .filter(release::Column::Repository.eq(repo.id))
            .one(db)
            .await
            .map(|model| model.map(Into::<RepositoryRelease>::into))
            .into_system_failure(),

        VersionOrUlid::Version(version) => RepositoryReleaseEntity::find()
            .filter(release::Column::Tag.eq(version.to_string()))
            .filter(release::Column::Repository.eq(repo.id))
            .one(db)
            .await
            .map(|model| model.map(Into::<RepositoryRelease>::into))
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getRepositoryReleaseWithAdditionalBounds", skip_all, fields(versionOrId = %version_or_id, repository.id = %repo.id, repository.name = %repo.name))]
pub async fn get_with_additional_bounds(
    db: &DbConn,
    repo: &Repository,
    version_or_id: VersionOrUlid,
    f: impl FnOnce(Select<RepositoryReleaseEntity>) -> Select<RepositoryReleaseEntity>,
) -> Result<Option<RepositoryRelease>, api::Response> {
    match version_or_id {
        VersionOrUlid::Ulid(id) => {
            f(RepositoryReleaseEntity::find_by_id(id).filter(release::Column::Repository.eq(repo.id)))
                .one(db)
                .await
                .map(|model| model.map(Into::<RepositoryRelease>::into))
                .into_system_failure()
        }

        VersionOrUlid::Version(version) => f(RepositoryReleaseEntity::find()
            .filter(release::Column::Tag.eq(version.to_string()))
            .filter(release::Column::Repository.eq(repo.id)))
        .one(db)
        .await
        .map(|model| model.map(Into::<RepositoryRelease>::into))
        .into_system_failure(),
    }
}
