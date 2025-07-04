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
use charted_database::entities::{ApiKeyEntity, apikey};
use charted_types::{ApiKey, NameOrUlid};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, Select};

#[instrument(name = "charted.server.ops.getApiKey", skip(db))]
pub async fn get(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<ApiKey>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => ApiKeyEntity::find_by_id(id)
            .one(db)
            .await
            .map(|model| model.map(Into::<ApiKey>::into))
            .into_system_failure(),

        NameOrUlid::Name(name) => ApiKeyEntity::find()
            .filter(apikey::Column::Name.eq(name))
            .one(db)
            .await
            .map(|model| model.map(Into::<ApiKey>::into))
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getApiKeyWithAdditionalBounds", skip(db, f))]
pub async fn get_with_additional_bounds(
    db: &DbConn,
    id_or_name: NameOrUlid,
    f: impl FnOnce(Select<ApiKeyEntity>) -> Select<ApiKeyEntity>,
) -> Result<Option<ApiKey>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => f(ApiKeyEntity::find_by_id(id))
            .one(db)
            .await
            .map(|model| model.map(Into::<ApiKey>::into))
            .into_system_failure(),

        NameOrUlid::Name(name) => f(ApiKeyEntity::find().filter(apikey::Column::Name.eq(name)))
            .one(db)
            .await
            .map(|model| model.map(Into::<ApiKey>::into))
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getApiKeyAsModel", skip(db))]
pub async fn get_as_model(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<apikey::Model>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => ApiKeyEntity::find_by_id(id).one(db).await.into_system_failure(),
        NameOrUlid::Name(name) => ApiKeyEntity::find()
            .filter(apikey::Column::Name.eq(name))
            .one(db)
            .await
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getApiKeyAsModelWithAdditionalBounds", skip(db, f))]
pub async fn get_as_model_with_additional_bounds(
    db: &DbConn,
    id_or_name: NameOrUlid,
    f: impl FnOnce(Select<ApiKeyEntity>) -> Select<ApiKeyEntity>,
) -> Result<Option<apikey::Model>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => f(ApiKeyEntity::find_by_id(id)).one(db).await.into_system_failure(),
        NameOrUlid::Name(name) => f(ApiKeyEntity::find().filter(apikey::Column::Name.eq(name)))
            .one(db)
            .await
            .into_system_failure(),
    }
}
