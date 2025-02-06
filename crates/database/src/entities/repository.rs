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
pub mod release;

use charted_types::{name::Name, ChartType, Repository, Ulid};
use sea_orm::{entity::prelude::*, sea_query::TableCreateStatement};
use sea_orm_migration::schema::*;

use super::{create_table, id};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "repositories")]
pub struct Model {
    pub description: Option<String>,
    pub deprecated: bool,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,

    #[sea_orm(column_type = "Text", nullable)]
    pub icon_hash: Option<String>,
    pub private: bool,
    pub creator: Option<Ulid>,
    pub owner: Ulid,
    pub name: Name,

    #[sea_orm(rename = "type")]
    pub type_: ChartType,

    #[sea_orm(column_type = "Text", primary_key, auto_increment = false)]
    pub id: Ulid,
}

impl From<Model> for Repository {
    fn from(model: Model) -> Self {
        Repository {
            description: model.description,
            deprecated: model.deprecated,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            icon_hash: model.icon_hash,
            private: model.private,
            creator: model.creator,
            owner: model.owner,
            name: model.name,
            type_: model.type_,
            id: model.id,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::repository::release::Entity")]
    Releases,
}

impl Related<super::repository::release::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Releases.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub(crate) enum Idens {
    #[sea_orm(iden = "repositories")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .if_not_exists()
        .col(string_len_null(Column::Description, 140))
        .col(boolean(Column::Deprecated).default(false))
        .col(text_null(Column::IconHash))
        .col(boolean(Column::Private).default(false))
        .col(text_null(Column::Creator))
        .col(text(Column::Owner))
        .col(string_len(Column::Name, 32))
        .col(enumeration(
            Column::Type,
            ChartType::name(),
            [ChartType::Application, ChartType::Library],
        ))
        .col(id())
        .to_owned()
}
