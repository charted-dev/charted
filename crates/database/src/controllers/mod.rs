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

pub mod apikeys;
pub mod member;
pub mod organization;
pub mod repository;
pub mod session;
pub mod users;

use crate::pagination::Request;
use charted_common::BoxedFuture;
use charted_core::pagination::Paginated;
use charted_entities::NameOrSnowflake;
use eyre::eyre;
use serde::Deserialize;
use sqlx::{database::HasArguments, query::Query, Postgres, Transaction};
use std::{any::Any, sync::Arc};

/// DbController is an abstraction on methods on querying, creating, updating, and
/// deleting data from the PostgreSQL database.
pub trait DbController: Send + Sync {
    /// Entity is the main datatype that this [`DbController`] controls.
    type Entity;

    /// `ID` is the type for the associated primary key for this [`DbController`]. In most cases,
    /// this will be a [`i64`], but sessions use a UUID as they're not a primary entity like
    /// most other entities; they're not special.
    type ID;

    /// Created is the main datatype that is passed through when creating the controller's
    /// main entity datatype.
    type Created: for<'de> Deserialize<'de>;

    /// Created is the main datatype that is passed through when patching the controller's
    /// main entity datatype.
    type Patched: for<'de> Deserialize<'de>;

    fn paginate(&self, _request: Request) -> BoxedFuture<eyre::Result<Paginated<Self::Entity>>> {
        Box::pin(async { Err(eyre!("Pagination is not supported by this database controller")) })
    }

    /// Returns a entity with a given ID. `Ok(None)` can be returned to indicate that
    /// the entity was not found.
    fn get(&self, id: Self::ID) -> BoxedFuture<eyre::Result<Option<Self::Entity>>>;

    /// Returns an entity with `S`, which can be a [Name][charted_entities::Name] or a snowflake ID. `Ok(None)` can
    /// be returned to indicate that the entity was not found.
    fn get_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(
        &'a self,
        nos: S,
    ) -> BoxedFuture<eyre::Result<Option<Self::Entity>>>;

    /// Inserts a new `Entity` with a given `Created` payload and a skeleton of what to use
    /// when inserting it.
    fn create<'a>(&'a self, payload: Self::Created, skeleton: &'a Self::Entity) -> BoxedFuture<eyre::Result<()>>;

    /// Patch a given `Entity` by its ID with the specified payload.
    fn patch(&self, id: Self::ID, payload: Self::Patched) -> BoxedFuture<eyre::Result<()>>;

    /// Deletes a `Entity` with their ID.
    fn delete(&self, id: Self::ID) -> BoxedFuture<eyre::Result<()>>;

    /// Check if `Entity` by their ID exists in the database.
    fn exists(&self, id: Self::ID) -> BoxedFuture<eyre::Result<bool>>;

    /// Check if `Entity` by the associated [`NameOrSnowflake`] exists in the database
    fn exists_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(&'a self, nos: S) -> BoxedFuture<eyre::Result<bool>>;
}

/// Represents a collection of database controllers. They are type-erased
/// to hack away the generic associated types and can be retrieve from
/// the [`Registry::get`] API.
#[derive(Clone)]
pub struct Registry(Vec<Arc<dyn Any + Send + Sync>>);

impl Registry {
    /// Creates a new [`Registry`] with no elements.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Registry {
        Registry(Vec::new())
    }

    /// Returns the amount of controllers that are avaliable
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this registry contains no controllers.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a database controller from type `D` if it was found. `None` is returned otherwise.
    pub fn get<D: DbController + 'static>(&self) -> Option<&D> {
        self.0.iter().filter_map(|x| x.downcast_ref()).next()
    }

    /// Inserts a new database controller.
    pub fn insert<D: DbController + 'static>(&mut self, controller: D) {
        self.0.push(Arc::new(controller));
    }
}

pub(crate) async fn perform_patch<
    F: for<'a> FnMut(
        Query<'a, Postgres, <Postgres as HasArguments<'a>>::Arguments>,
    ) -> Query<'a, Postgres, <Postgres as HasArguments<'a>>::Arguments>,
>(
    txn: &mut Transaction<'_, Postgres>,
    sql: &str,
    mut build: F,
) -> eyre::Result<()> {
    let query = build(sqlx::query(sql));
    match query.execute(&mut **txn).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!(query = sql, "unable to execute query");
            sentry::capture_error(&e);

            Err(Into::into(e))
        }
    }
}

macro_rules! perform_patching_impl {
    (optional [$txn:expr, $payload:expr]: table $table:literal, column $column:literal, where id = $id:expr; if |$value:ident| $cond:expr) => {
        match ($payload).as_ref() {
            Some(content) => {
                let $value = content;

                // `false` = apply, `true` = reset
                let query = if !$cond {
                    concat!("update ", $table, " set ", $column, " = $1 where id = $2;")
                } else {
                    concat!("update ", $table, " set ", $column, " = NULL where id = $2;")
                };

                perform_patch(&mut $txn, query, |query| {
                    if !$cond {
                        query.bind($id)
                    } else {
                        query.bind(content.clone()).bind($id)
                    }
                })
                .await?;
            }

            // don't perform the update
            None => {}
        }
    };

    ([$txn:expr, $payload:expr]: table $table:literal, column $column:literal, where id = $id:expr; if |$value:ident| $cond:expr) => {{
        let $value = $payload;
        let query = if !$cond {
            concat!("update ", $table, " set ", $column, " = $1 where id = $2;")
        } else {
            concat!("update ", $table, " set ", $column, " = NULL where id = $1;")
        };

        perform_patch(&mut $txn, query, |query| {
            if !$cond {
                query.bind($id)
            } else {
                query.bind($value).bind($id)
            }
        })
        .await?;
    }};
}

pub(crate) use perform_patching_impl as patch;

#[cfg(test)]
mod tests {
    use super::*;

    struct MyController;
    impl super::DbController for MyController {
        type Entity = ();
        type Created = ();
        type Patched = ();
        type ID = i64;

        fn get(&self, _id: i64) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
            todo!()
        }

        fn get_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(
            &'a self,
            _nos: S,
        ) -> BoxedFuture<eyre::Result<Option<Self::Entity>>> {
            todo!()
        }

        fn create(&self, _payload: Self::Created, _skeleton: &Self::Entity) -> BoxedFuture<eyre::Result<()>> {
            todo!()
        }

        fn delete(&self, _id: i64) -> BoxedFuture<eyre::Result<()>> {
            todo!()
        }

        fn exists(&self, _id: i64) -> BoxedFuture<eyre::Result<bool>> {
            todo!()
        }

        fn exists_by<'a, S: Into<NameOrSnowflake> + Send + 'a>(&'a self, _nos: S) -> BoxedFuture<eyre::Result<bool>> {
            todo!()
        }

        fn patch(&self, _id: i64, _payload: Self::Patched) -> BoxedFuture<eyre::Result<()>> {
            todo!()
        }
    }

    #[test]
    fn test_registry() {
        let mut registry = Registry::new();
        registry.insert(MyController);

        assert!(registry.get::<MyController>().is_some());
    }
}
