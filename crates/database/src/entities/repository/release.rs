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

use charted_types::{RepositoryRelease, Ulid, Version};
use sea_orm::{
    entity::prelude::*,
    sea_query::{ForeignKey, TableCreateStatement},
};
use sea_orm_migration::schema::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "repository_releases")]
pub struct Model {
    #[sea_orm(column_type = "Text", nullable)]
    pub update_text: Option<String>,
    pub repository: Ulid,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    pub yanked: bool,
    pub title: Option<String>,

    #[sea_orm(column_type = "Text")]
    pub tag: Version,

    #[sea_orm(column_type = "Text", primary_key, auto_increment = false)]
    pub id: Ulid,
}

impl From<Model> for RepositoryRelease {
    fn from(model: Model) -> Self {
        RepositoryRelease {
            update_text: model.update_text,
            repository: model.repository,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            yanked: model.yanked,
            title: model.title,
            tag: model.tag,
            id: model.id,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::Entity",
        from = "Column::Repository",
        to = "super::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Repository,
}

impl Related<super::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repository.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(DeriveIden)]
pub(crate) enum Idens {
    #[sea_orm(iden = "repository_releases")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    table_auto(Idens::Table)
        .if_not_exists()
        .col(text_null(Column::UpdateText))
        .col(text(Column::Repository))
        .col(boolean(Column::Yanked))
        .col(string_len_null(Column::Title, 32))
        .col(text(Column::Tag))
        .col(text(Column::Id).primary_key())
        .foreign_key(
            ForeignKey::create()
                .name("fk_repository_release_owner")
                .from(Idens::Table, Column::Repository)
                .to(super::Idens::Table, super::Column::Id),
        )
        .to_owned()
}
