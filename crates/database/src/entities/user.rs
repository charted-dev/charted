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

use super::create_table;
use charted_types::{Ulid, User, name::Name};
use sea_orm::{entity::prelude::*, sea_query::TableCreateStatement};
use sea_orm_migration::schema::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub verified_publisher: bool,
    pub prefers_gravatar: bool,

    #[sea_orm(column_type = "Text", nullable)]
    pub gravatar_email: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub avatar_hash: Option<String>,

    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    pub username: Name,
    pub password: Option<String>,
    pub email: String,
    pub admin: bool,
    pub name: Option<String>,

    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: Ulid,
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        User {
            verified_publisher: model.verified_publisher,
            prefers_gravatar: model.prefers_gravatar,
            gravatar_email: model.gravatar_email,
            description: model.description,
            avatar_hash: model.avatar_hash,
            created_at: model.created_at.into(),
            updated_at: model.created_at.into(),
            username: model.username,
            admin: model.admin,
            name: model.name,
            id: model.id,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::apikey::Entity")]
    ApiKey,

    #[sea_orm(has_many = "super::organization::Entity")]
    Organization,

    #[sea_orm(has_many = "super::session::Entity")]
    Session,

    #[sea_orm(has_one = "super::user_connections::Entity")]
    UserConnection,
}

impl Related<super::user_connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserConnection.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub(crate) enum Idens {
    #[sea_orm(iden = "users")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .if_not_exists()
        .col(boolean(Column::VerifiedPublisher))
        .col(boolean(Column::PrefersGravatar))
        .col(text_null(Column::GravatarEmail))
        .col(string_len_null(Column::Description, 240))
        .col(text_null(Column::AvatarHash))
        .col(string_len_uniq(Column::Username, 64))
        .col(text_null(Column::Password))
        .col(text(Column::Email))
        .col(boolean(Column::Admin))
        .col(string_len_null(Column::Name, 64))
        .col(text(Column::Id).primary_key())
        .to_owned()
}
