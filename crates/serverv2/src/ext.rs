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

use crate::Env;
use charted_core::BoxedFuture;
use charted_database::entities::{OrganizationEntity, UserEntity, organization, user};
use charted_types::{NameOrUlid, Organization, Owner, Ulid, User, name::Name};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

/// Extension trait for [`Owner`].
pub trait OwnerExt: Sized {
    /// Queries a [`Owner`] by either their ID ([`Ulid`](NameOrUlid::Ulid)) or by their
    /// name ([`Name`](NameOrUlid::Name)).
    fn query_by_id_or_name<'s>(
        env: &'s Env,
        id_or_name: NameOrUlid,
    ) -> BoxedFuture<'s, Result<Option<Owner>, sea_orm::DbErr>> {
        match id_or_name {
            NameOrUlid::Name(name) => Box::pin(Self::query_by_name(env, name)),
            NameOrUlid::Ulid(id) => Box::pin(Self::query_by_id(env, id)),
        }
    }

    /// Queries a [`Owner`] by their [name](Name).
    fn query_by_name<'s>(
        ctx: &'s Env,
        name: Name,
    ) -> impl Future<Output = Result<Option<Owner>, sea_orm::DbErr>> + Send + 's;

    /// Queries a [`Owner`] by their [id](Ulid).
    fn query_by_id<'s>(
        ctx: &'s Env,
        id: Ulid,
    ) -> impl Future<Output = Result<Option<Owner>, sea_orm::DbErr>> + Send + 's;
}

impl OwnerExt for Owner {
    async fn query_by_name(env: &Env, name: Name) -> Result<Option<Owner>, sea_orm::DbErr> {
        if let Some(user) = UserEntity::find()
            .filter(user::Column::Username.eq(name.clone()))
            .one(&env.db)
            .await?
            .map(Into::<User>::into)
        {
            return Ok(Some(Owner::User(user)));
        }

        if let Some(org) = OrganizationEntity::find()
            .filter(organization::Column::Name.eq(name))
            .one(&env.db)
            .await?
            .map(Into::<Organization>::into)
        {
            return Ok(Some(Owner::Organization(org)));
        }

        Ok(None)
    }

    async fn query_by_id(env: &Env, id: Ulid) -> Result<Option<Owner>, sea_orm::DbErr> {
        if let Some(user) = UserEntity::find()
            .filter(user::Column::Id.eq(id))
            .one(&env.db)
            .await?
            .map(Into::<User>::into)
        {
            return Ok(Some(Owner::User(user)));
        }

        if let Some(org) = OrganizationEntity::find()
            .filter(organization::Column::Id.eq(id))
            .one(&env.db)
            .await?
            .map(Into::<Organization>::into)
        {
            return Ok(Some(Owner::Organization(org)));
        }

        Ok(None)
    }
}
