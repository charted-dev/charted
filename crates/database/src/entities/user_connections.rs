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
use charted_types::{Ulid, UserConnections};
use sea_orm::{
    entity::prelude::*,
    sea_query::{ForeignKey, TableCreateStatement},
};
use sea_orm_migration::schema::*;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_connections")]
pub struct Model {
    #[sea_orm(column_type = "Text", nullable)]
    pub noelware_account_id: Option<Ulid>,

    #[sea_orm(column_type = "Text", nullable)]
    pub google_account_id: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub github_account_id: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub gitlab_account_id: Option<String>,

    pub created_at: ChronoDateTimeUtc,
    pub updated_at: ChronoDateTimeUtc,
    pub account: Ulid,

    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: Ulid,
}

impl From<Model> for UserConnections {
    fn from(model: Model) -> Self {
        UserConnections {
            noelware_account_id: model.noelware_account_id,
            google_account_id: model.google_account_id,
            github_account_id: model.github_account_id,
            gitlab_account_id: model.gitlab_account_id,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
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
    #[sea_orm(iden = "user_connections")]
    Table,
}

pub(crate) fn table() -> TableCreateStatement {
    create_table(Idens::Table)
        .if_not_exists()
        .col(text_null(Column::NoelwareAccountId))
        .col(text_null(Column::GoogleAccountId))
        .col(text_null(Column::GithubAccountId))
        .col(text_null(Column::GitlabAccountId))
        .col(text_null(Column::Account))
        .col(id())
        .foreign_key(
            ForeignKey::create()
                .name("fk_user_connections_account")
                .from(Idens::Table, Column::Account)
                .to(super::user::Idens::Table, super::user::Column::Id),
        )
        .to_owned()
}
