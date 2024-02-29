// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

pub mod controllers;

use crate::config::Config;
use sqlx::{
    migrate,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use std::{str::FromStr, time::Duration};

/// A static [`Migrator`] instance for migrations that are embedded
/// in this crate.
pub static MIGRATIONS: Migrator = migrate!();

/// Creates a [`PgPool`] and runs the migrations, if explicitlly enabled.
pub async fn create_pool(config: &Config) -> eyre::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect_with(
            PgConnectOptions::from_str(&config.database.to_string())?
                .application_name("charted-server")
                .log_statements(tracing::log::LevelFilter::Trace)
                .log_slow_statements(tracing::log::LevelFilter::Warn, Duration::from_secs(1)),
        )
        .await?;

    if config.database.run_migrations {
        let span = info_span!("charted.db.migrations.run");
        let _ = span.enter();

        info!("running all db migrations!");
        MIGRATIONS.run(&pool).await?;

        info!("ran all db migrations successfully");
    }

    Ok(pool)
}

/// Macro to implement the `paginate` method for a controller.
macro_rules! impl_paginate_priv {
    ("repositories") => {
        fn paginate<'life0, 'async_trait>(
            &'life0 self,
            request: $crate::db::controllers::PaginationRequest
        ) -> ::core::pin::Pin<
            Box<
                dyn ::std::future::Future<
                    Output = ::eyre::Result<$crate::server::pagination::Pagination<$crate::common::models::entities::Repository>>> + ::core::marker::Send + 'async_trait
                >
            >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            use ::sqlx::{Row, FromRow};
            use ::eyre::Context;

            Box::pin(async move {
                let mut query = ::sqlx::QueryBuilder::<::sqlx::Postgres>::new("select repositories.* from repositories ");
                if let Some(cursor) = request.cursor {
                    query.push("where repositories.id <= ");
                    query.push_bind(i64::try_from(cursor).unwrap());
                    query.push(" and ");
                } else {
                    query.push("where ");
                }

                let owner_id = request.owner_id.unwrap_or_else(|| panic!("INTERNAL BUG: missing `owner_id`"));
                query.push("repositories.owner = ");
                query.push_bind(i64::try_from(owner_id).unwrap());
                query.push(" ");

                if !request.list_private_stuff {
                    query.push(" and repositories.private = false ");
                }

                match request.order_by {
                    $crate::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    $crate::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1).push(" ");

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(entries) => {
                        // if the cursor is less than the actual entries, we can't iterate
                        // over more pages.
                        let cursor = if entries.len() < request.per_page {
                            None
                        } else {
                            entries
                                .last()
                                .map(|entry| entry.get::<i64, _>("id"))
                                .map(|e| e as u64)
                        };

                        let page_info = $crate::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$crate::common::models::entities::Repository>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok($crate::server::pagination::Pagination { page_info, data })
                    }

                    Err(e) => {
                        ::tracing::error!(error = %e, concat!("unable to complete pagination request for table [repositories]"));
                        ::sentry::capture_error(&e);

                        Err(e).context(concat!("unable to complete pagination request for table [repositories]"))
                    }
                }
            })
        }
    };

    ("organizations") => {
        fn paginate<'life0, 'async_trait>(&'life0 self, request: $crate::db::controllers::PaginationRequest) -> ::core::pin::Pin<
            Box<
                dyn ::std::future::Future<
                    Output = ::eyre::Result<
                        $crate::server::pagination::Pagination<
                            $crate::common::models::entities::Organization
                        >
                    >
                > + ::core::marker::Send + 'async_trait
            >
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            use ::sqlx::{Row, FromRow};
            use ::eyre::Context;

            Box::pin(async move {
                let mut query = ::sqlx::QueryBuilder::<::sqlx::Postgres>::new("select organizations.* from organizations ");
                if let Some(cursor) = request.cursor {
                    query.push("where organizations.id <= ");
                    query.push_bind(i64::try_from(cursor).unwrap());
                    query.push(" and ");
                } else {
                    query.push("where ");
                }

                let owner_id = request.owner_id.unwrap_or_else(|| panic!("INTERNAL BUG: missing `owner_id`"));
                query.push("organizations.owner = ");
                query.push_bind(i64::try_from(owner_id).unwrap());
                query.push(" ");

                if !request.list_private_stuff {
                    query.push(" and organizations.private = false ");
                }

                match request.order_by {
                    $crate::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    $crate::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1).push(" ");

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(entries) => {
                        // if the cursor is less than the actual entries, we can't iterate
                        // over more pages.
                        let cursor = if entries.len() < request.per_page {
                            None
                        } else {
                            entries
                                .last()
                                .map(|entry| entry.get::<i64, _>("id"))
                                .map(|e| e as u64)
                        };

                        let page_info = $crate::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$crate::common::models::entities::Organization>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok($crate::server::pagination::Pagination { page_info, data })
                    }

                    Err(e) => {
                        ::tracing::error!(error = %e, concat!("unable to complete pagination request for table [organizatins]"));
                        ::sentry::capture_error(&e);

                        Err(e).context(concat!("unable to complete pagination request for table [organizations]"))
                    }
                }
            })
        }
    };

    ($table:literal as $ty:ty) => {
        fn paginate<'life0, 'async_trait>(&'life0 self, request: $crate::db::controllers::PaginationRequest) -> ::core::pin::Pin<
            Box<dyn ::std::future::Future<Output = ::eyre::Result<$crate::server::pagination::Pagination<$ty>>> + ::core::marker::Send + 'async_trait>
        >
        where
            'life0: 'async_trait,
            Self: 'async_trait,
        {
            use ::sqlx::{Row, FromRow};
            use ::eyre::Context;

            Box::pin(async move {
                let mut query = ::sqlx::QueryBuilder::<::sqlx::Postgres>::new(concat!("select ", $table, ".* from ", $table, " "));
                if let Some(cursor) = request.cursor {
                    query.push("where id <= ");
                    query.push_bind(i64::try_from(cursor).unwrap());
                    query.push(" ");
                }

                match request.order_by {
                    $crate::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    $crate::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1);

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(entries) => {
                        // if the cursor is less than the actual entries, we can't iterate
                        // over more pages.
                        let cursor = if entries.len() < request.per_page {
                            None
                        } else {
                            entries
                                .last()
                                .map(|entry| entry.get::<i64, _>("id"))
                                .map(|e| e as u64)
                        };

                        let page_info = $crate::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$ty>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok($crate::server::pagination::Pagination { page_info, data })
                    }

                    Err(e) => {
                        ::tracing::error!(error = %e, concat!("unable to complete pagination request for table [", $table, "]"));
                        ::sentry::capture_error(&e);

                        Err(e).context(concat!("unable to complete pagination request for table [", $table, "]"))
                    }
                }
            })
        }
    };
}

pub(crate) use impl_paginate_priv as impl_paginate;

macro_rules! impl_patch_for_priv {
    // Syntax:
    //
    //      impl_patch_for!([txn]: update on [payload.description] on table "users" and column "description" where id = 1234);
    //
    ([$txn:expr]: update on [$payload:expr] in table $table:literal, in column $column:literal where id = $id:expr$(; if |$val:ident| $cond:expr)?) => {
        $crate::db::impl_patch_for!($txn; {
            payload: $payload;
            column:  $column;
            table:   $table;
            id:      $id;
        }$(; if |$val| $cond)?);
    };

    // Syntax:
    //
    //     impl_patch_for!(txn; {
    //         payload: payload.description;
    //         column:  "description";
    //         table:   "users";
    //         id:      payload.id;
    //     }; if |val| if val.is_empty());
    //
    ($txn:expr; {
        payload: $payload:expr;
        column:  $column:literal;
        table:   $table:literal;
        id:      $id:expr;
    }; if |$value:ident| $cond:expr) => {
        match $payload {
            Some(ref val) => {
                let $value = val;
                if $cond {
                    match sqlx::query(concat!("update ", $table, " set ", $column, " = $1 where id = $2;"))
                        .bind($value)
                        .bind($id)
                        .execute(&mut *$txn)
                        .await {
                            Ok(_) => {}
                            Err(e) => {
                                ::tracing::error!(id = $id, error = %e, concat!("unable to update column [", $column, "] for table [", $table, "]"));
                                ::sentry::capture_error(&e);

                                drop($txn);
                                return Err(e.into());
                            }
                        }
                }
            }

            None => {}
        }
    };

    ($txn:expr; {
        payload: $payload:expr;
        column:  $column:literal;
        table:   $table:literal;
        id:      $id:expr;
    }) => {
        match $payload {
            // if `content` is empty, it is a indication that it should be wiped out
            Some(ref content) if content.is_empty() => {
                match sqlx::query(concat!("update ", $table, " set ", $column, " = NULL where id = $1;"))
                    .bind($id)
                    .execute(&mut *$txn)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        ::tracing::error!(
                            user.id = $id,
                            error = %e,
                            table = $table,
                            column = $column,
                            "unable to update column entry in table to NULL"
                        );

                        ::sentry::capture_error(&e);

                        // drop the transaction as sqlx will rollback the transaction state
                        drop($txn);
                        return Err(e.into());
                    }
                }
            }

            // Perform the update anyway
            Some(ref content) => {
                match sqlx::query(concat!("update ", $table, " set ", $column, " = $1 where id = $2;"))
                    .bind(content)
                    .bind($id)
                    .execute(&mut *$txn)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        ::tracing::error!(
                            user.id = $id,
                            error = %e,
                            table = $table,
                            column = $column,
                            "unable to update column entry in table"
                        );

                        ::sentry::capture_error(&e);

                        // drop the transaction as sqlx will rollback the transaction state
                        drop($txn);
                        return Err(e.into());
                    }
                }
            }

            // don't perform the update
            None => {}
        }
    }
}

pub(crate) use impl_patch_for_priv as impl_patch_for;
