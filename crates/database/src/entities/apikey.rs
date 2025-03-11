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

use super::{create_table, id};
use charted_types::{ApiKey, Ulid, name::Name};
use sea_orm::{entity::prelude::*, sea_query::TableCreateStatement};
use sea_orm_migration::schema::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "apikeys")]
pub struct Model {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub expires_in: Option<ChronoDateTimeUtc>,
    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    pub scopes: i64,
    pub owner: Ulid,
    pub token: String,
    pub name: Name,

    #[sea_orm(column_type = "Text", primary_key, auto_increment = false)]
    pub id: Ulid,
}

impl From<Model> for ApiKey {
    fn from(model: Model) -> Self {
        ApiKey {
            display_name: model.display_name,
            description: model.description,
            expires_in: model.expires_in.map(Into::into),
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
            scopes: model.scopes,
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
        on_delete = "Cascade"
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
    #[sea_orm(iden = "apikeys")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .col(string_len_null(Column::DisplayName, 32))
        .col(string_len_null(Column::Description, 140))
        .col(timestamp_null(Column::ExpiresIn))
        .col(big_integer(Column::Scopes))
        .col(text(Column::Token))
        .col(text(Column::Owner))
        .col(Name::into_column(Column::Name))
        .col(id())
        .to_owned()
}
