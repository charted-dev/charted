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

use crate::entities;
use charted_types::ChartType;
use sea_orm::{sea_query::extension::postgres::Type, ActiveEnum};
use sea_orm_migration::prelude::*;

pub fn migration() -> impl MigrationTrait {
    Impl
}

struct Impl;

impl MigrationName for Impl {
    fn name(&self) -> &str {
        "init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Impl {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ChartType::name())
                    .values([ChartType::Application, ChartType::Library])
                    .to_owned(),
            )
            .await?;

        manager.create_table(entities::user::table()).await?;
        manager.create_table(entities::user_connections::table()).await?;
        manager.create_table(entities::session::table()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(entities::user::Idens::Table)
                    .table(entities::user_connections::Idens::Table)
                    .table(entities::session::Idens::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(ChartType::name()).cascade().to_owned())
            .await
    }
}
