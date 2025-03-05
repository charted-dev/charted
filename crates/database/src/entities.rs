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

pub mod apikey;
pub mod organization;
pub mod repository;
pub mod session;
pub mod user;
pub mod user_connections;

pub use apikey::Entity as ApiKeyEntity;
pub use repository::{release::Entity as RepositoryReleaseEntity, Entity as RepositoryEntity};
use sea_orm::{
    prelude::Expr,
    sea_query::{ColumnDef, IntoIden, Table, TableCreateStatement},
    DeriveIden,
};
use sea_orm_migration::schema::{text, timestamp};
pub use session::Entity as SessionEntity;
pub use user::Entity as UserEntity;
pub use user_connections::Entity as UserConnectionsEntity;

#[derive(DeriveIden)]
#[sea_orm(rename_all = "snake_case")]
enum Idens {
    CreatedAt,
    UpdatedAt,
    Id,
}

/// Utility function like [`table_auto`][sea_orm_migration::schema::table_auto] but uses
/// `snake_case` on `created_at` and `updated_at` fields.
pub(in crate::entities) fn create_table<T: IntoIden + 'static>(name: T) -> TableCreateStatement {
    Table::create()
        .table(name)
        .if_not_exists()
        .col(timestamp(Idens::CreatedAt).default(Expr::current_timestamp()))
        .col(timestamp(Idens::UpdatedAt).default(Expr::current_timestamp()))
        .take()
}

/// Returns a [column definition][ColumnDef] that returns `id TEXT NOT NULL PRIMARY KEY`.
pub(in crate::entities) fn id() -> ColumnDef {
    text(Idens::Id).primary_key().take()
}
