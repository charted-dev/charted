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

use charted_types::{Session, Ulid};
use sea_orm::{
    entity::prelude::*,
    sea_query::{ForeignKey, TableCreateStatement},
};
use sea_orm_migration::schema::*;

use super::{create_table, id};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(column_type = "Text")]
    pub refresh_token: String,

    #[sea_orm(column_type = "Text")]
    pub access_token: String,

    #[sea_orm(column_type = "Text")]
    pub account: Ulid,

    #[sea_orm(column_type = "Text", primary_key, auto_increment = false)]
    pub id: Ulid,
}

impl From<Model> for Session {
    fn from(model: Model) -> Self {
        Session {
            refresh_token: Some(model.refresh_token),
            access_token: Some(model.access_token),
            owner: model.account,
            id: model.id,
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::Account",
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
    #[sea_orm(iden = "sessions")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .if_not_exists()
        .col(text(Column::RefreshToken))
        .col(text(Column::AccessToken))
        .col(text(Column::Account))
        .col(id())
        .foreign_key(
            ForeignKey::create()
                .name("fk_session_account")
                .from(Idens::Table, Column::Account)
                .to(super::user::Idens::Table, super::user::Column::Id),
        )
        .to_owned()
}
