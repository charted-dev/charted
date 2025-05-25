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
use charted_database::entities::{OrganizationEntity, organization};
use charted_types::{NameOrUlid, Organization};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, Select};

#[instrument(name = "charted.server.ops.findOrganization", skip_all)]
pub async fn find(
    db: &DbConn,
    f: impl FnOnce() -> Select<OrganizationEntity>,
) -> Result<Option<organization::Model>, api::Response> {
    f().one(db).await.into_system_failure()
}

pub async fn get(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<Organization>, api::Response> {
    as_model(db, id_or_name).await.map(|model| model.map(Into::into))
}

pub async fn as_model(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<organization::Model>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => find(db, || OrganizationEntity::find_by_id(id)).await,
        NameOrUlid::Name(name) => {
            find(db, || {
                OrganizationEntity::find().filter(organization::Column::Name.eq(name))
            })
            .await
        }
    }
}

pub async fn as_model_with_additional_bounds(
    db: &DbConn,
    id_or_name: NameOrUlid,
    f: impl FnOnce(Select<OrganizationEntity>) -> Select<OrganizationEntity>,
) -> Result<Option<organization::Model>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => find(db, || f(OrganizationEntity::find_by_id(id))).await,
        NameOrUlid::Name(name) => {
            find(db, || {
                f(OrganizationEntity::find().filter(organization::Column::Name.eq(name)))
            })
            .await
        }
    }
}
