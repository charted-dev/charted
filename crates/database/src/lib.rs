// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

pub mod controller;

use sqlx::{migrate, migrate::Migrator};

/// A static [`Migrator`] instance for migrations that are embedded
/// in this crate.
pub static MIGRATIONS: Migrator = migrate!();

/// Macro to implement the `paginate` method for a controller.
///
/// ## Example
/// ```no_run
/// # use async_trait::async_trait;
/// # use charted_database::{DbController, impl_paginate};
/// #
/// # pub struct MyDbController { pool: ::sqlx::PgPool }
/// #
/// #[async_trait]
/// impl DbController for MyDbController {
///     impl_paginate!("table" -> MyDbType);
/// }
/// ```
macro_rules! impl_paginate_priv {
    ("repositories" -> $ty:ty) => {
        fn paginate<'life0, 'async_trait>(&'life0 self, request: $crate::controller::PaginationRequest) -> ::core::pin::Pin<
            Box<dyn ::std::future::Future<Output = ::eyre::Result<::charted_common::server::pagination::Pagination<$ty>>> + ::core::marker::Send + 'async_trait>
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
                    query.push_bind(cursor as i64);
                    query.push(" and ");
                } else {
                    query.push("where ");
                }

                let owner_id = request.owner_id.unwrap_or_else(|| panic!("INTERNAL BUG: missing `owner_id`"));
                query.push("repositories.owner = ");
                query.push_bind(owner_id as i64);
                query.push(" ");

                match request.order_by {
                    ::charted_common::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    ::charted_common::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1).push(" ");

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(mut entries) => {
                        let cursor = entries
                            .pop()
                            .map(|entry| entry.get::<i64, _>("id"))
                            .map(|e| e as u64);

                        let page_info = ::charted_common::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$ty>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok(::charted_common::server::pagination::Pagination { page_info, data })
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

    ("organizations" -> $ty:ty) => {
        fn paginate<'life0, 'async_trait>(&'life0 self, request: $crate::controller::PaginationRequest) -> ::core::pin::Pin<
            Box<dyn ::std::future::Future<Output = ::eyre::Result<::charted_common::server::pagination::Pagination<$ty>>> + ::core::marker::Send + 'async_trait>
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
                    query.push("where id <= ");
                    query.push_bind(cursor as i64);
                    query.push(" and ");
                } else {
                    query.push("where ");
                }

                let owner_id = request.owner_id.unwrap_or_else(|| panic!("INTERNAL BUG: missing `owner_id`"));
                query.push("owner_id = ");
                query.push_bind(owner_id as i64);
                query.push(" ");

                match request.order_by {
                    ::charted_common::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    ::charted_common::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1).push(" ");

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(mut entries) => {
                        let cursor = entries
                            .pop()
                            .map(|entry| entry.get::<i64, _>("id"))
                            .map(|e| e as u64);

                        let page_info = ::charted_common::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$ty>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok(::charted_common::server::pagination::Pagination { page_info, data })
                    }

                    Err(e) => {
                        ::tracing::error!(error = %e, "unable to complete pagination request for table [organizations]");
                        ::sentry::capture_error(&e);

                        Err(e).context("unable to complete pagination request for table [organizations]")
                    }
                }
            })
        }
    };

    ($table:literal -> $ty:ty) => {
        fn paginate<'life0, 'async_trait>(&'life0 self, request: $crate::controller::PaginationRequest) -> ::core::pin::Pin<
            Box<dyn ::std::future::Future<Output = ::eyre::Result<::charted_common::server::pagination::Pagination<$ty>>> + ::core::marker::Send + 'async_trait>
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
                    query.push_bind(cursor as i64);
                    query.push(" ");
                }

                match request.order_by {
                    ::charted_common::server::pagination::OrderBy::Ascending => query.push("order by id ASC "),
                    ::charted_common::server::pagination::OrderBy::Descending => query.push("order by id DESC "),
                };

                query.push("limit ").push_bind((request.per_page as i32) + 1);

                let query = query.build();
                match query.fetch_all(&self.pool).await {
                    Ok(mut entries) => {
                        let cursor = entries
                            .pop()
                            .map(|entry| entry.get::<i64, _>("id"))
                            .map(|e| e as u64);

                        let page_info = ::charted_common::server::pagination::PageInfo { cursor };
                        let data = entries.iter().filter_map(|row| <$ty>::from_row(row).ok()).collect::<::std::vec::Vec::<_>>();

                        Ok(::charted_common::server::pagination::Pagination { page_info, data })
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
    ($txn:expr, $payload:expr, $id:expr => {
        table -> $table:literal;
        entry -> $entry:literal;
    }) => {
        if let Some(val) = $payload {
            match sqlx::query(concat!("update ", $table, " set ", $entry, " = $1 where id = $2"))
                .bind(val)
                .bind($id)
                .execute(&mut *$txn)
                .await
                .context(concat!("unable to update [", $entry, "] for table [", $table, "]"))
            {
                Ok(_) => {}
                Err(e) => {
                    ::tracing::error!(id = $id, error = %e, concat!("unable to update [", $entry, "] for table [", $table, "]"));
                    ::sentry::capture_error(&*e);

                    // drop it so it can be rolled back.
                    ::std::mem::drop($txn);
                    return Err(e);
                }
            }
        }
    };

    ($txn:expr, $payload:expr, $id:expr => {
        table -> $table:literal;
        entry -> $entry:literal;
        value -> $value:expr;
    }) => {
        if let Some(_) = $payload {
            match sqlx::query(concat!("update ", $table, " set ", $entry, " = $1 where id = $2"))
                .bind($value)
                .bind($id)
                .execute(&mut *$txn)
                .await
                .context(concat!("unable to update [", $entry, "] for table [", $table, "]"))
            {
                Ok(_) => {}
                Err(e) => {
                    ::tracing::error!(id = $id, error = %e, concat!("unable to update [", $entry, "] for table [", $table, "]"));
                    ::sentry::capture_error(&*e);

                    // drop it so it can be rolled back.
                    ::std::mem::drop($txn);
                    return Err(e);
                }
            }
        }
    };
}

pub(crate) use impl_patch_for_priv as impl_patch_for;
