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

pub mod member;

use charted_types::{name::Name, Organization, Ulid};
use sea_orm::{entity::prelude::*, sea_query::TableCreateStatement};
use sea_orm_migration::schema::*;

use super::{create_table, id};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    pub verified_publisher: bool,
    pub prefers_gravatar: bool,

    #[sea_orm(column_type = "Text", nullable)]
    pub gravatar_email: Option<String>,
    pub display_name: Option<String>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,

    #[sea_orm(column_type = "Text", nullable)]
    pub icon_hash: Option<String>,
    pub private: bool,
    pub owner: Ulid,
    pub name: Name,

    #[sea_orm(column_type = "Text", primary_key, auto_increment = false)]
    pub id: Ulid,
}

impl From<Model> for Organization {
    fn from(model: Model) -> Self {
        Organization {
            verified_publisher: model.verified_publisher,
            prefers_gravatar: model.prefers_gravatar,
            gravatar_email: model.gravatar_email,
            display_name: model.display_name,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            icon_hash: model.icon_hash,
            private: model.private,
            owner: model.owner,
            name: model.name,
            id: model.id,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Owner",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub(crate) enum Idens {
    #[sea_orm(rename = "organizations")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .col(boolean(Column::VerifiedPublisher).default(false))
        .col(boolean(Column::PrefersGravatar).default(false))
        .col(text_null(Column::GravatarEmail))
        .col(string_len_null(Column::DisplayName, 32))
        .col(boolean(Column::Private).default(false))
        .col(text(Column::IconHash))
        .col(Name::into_column(Column::Owner))
        .col(Name::into_column(Column::Name))
        .col(id())
        .to_owned()
}
