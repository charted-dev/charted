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
use charted_database::entities::{UserEntity, user};
use charted_types::{NameOrUlid, User};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, Select};

#[instrument(name = "charted.server.ops.findUser", skip_all)]
pub async fn find(
    db: &DbConn,
    f: impl FnOnce(Select<UserEntity>) -> Select<UserEntity>,
) -> Result<Option<user::Model>, api::Response> {
    f(UserEntity::find()).one(db).await.into_system_failure()
}

#[instrument(name = "charted.server.ops.getUser", skip(db))]
pub async fn get(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<User>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => UserEntity::find_by_id(id)
            .one(db)
            .await
            .map(|model| model.map(Into::<User>::into))
            .into_system_failure(),

        NameOrUlid::Name(name) => UserEntity::find()
            .filter(user::Column::Username.eq(name.clone()))
            .one(db)
            .await
            .map(|model| model.map(Into::<User>::into))
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getUserAsModel", skip(db))]
pub async fn get_as_model(db: &DbConn, id_or_name: NameOrUlid) -> Result<Option<user::Model>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => UserEntity::find_by_id(id).one(db).await.into_system_failure(),
        NameOrUlid::Name(name) => UserEntity::find()
            .filter(user::Column::Username.eq(name.clone()))
            .one(db)
            .await
            .into_system_failure(),
    }
}

#[instrument(name = "charted.server.ops.getUserWithModel", skip(db))]
pub async fn get_with_model(
    db: &DbConn,
    id_or_name: NameOrUlid,
) -> Result<Option<(User, user::Model)>, api::Response> {
    match id_or_name {
        NameOrUlid::Ulid(id) => UserEntity::find_by_id(id)
            .one(db)
            .await
            .map(|model| model.map(|model| (model.clone().into(), model)))
            .into_system_failure(),

        NameOrUlid::Name(name) => UserEntity::find()
            .filter(user::Column::Username.eq(name.clone()))
            .one(db)
            .await
            .map(|model| model.map(|model| (model.clone().into(), model)))
            .into_system_failure(),
    }
}
