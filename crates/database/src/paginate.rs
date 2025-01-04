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

use crate::{DbConnection, MultiBackend};
use diesel::{
    query_builder::{AstPass, Query, QueryFragment, QueryId},
    query_dsl::methods::LoadQuery,
    sql_types::BigInt,
    QueryResult, RunQueryDsl,
};

/// Amount of elements that can be in per page.
pub const PER_PAGE: i64 = 10;

/// Trait that implements for paginating queries.
pub trait Paginate: Sized {
    /// Create a new [paginated][Paginated] query based off the current `page`.
    fn paginate(self, page: i64) -> PaginatedQuery<Self> {
        PaginatedQuery {
            query: self,
            per_page: PER_PAGE,
            page,
            offset: (page - 1) * PER_PAGE,
        }
    }
}

#[derive(Debug, Clone, QueryId)]
pub struct PaginatedQuery<T> {
    query: T,
    per_page: i64,
    page: i64,
    offset: i64,
}

impl<T> PaginatedQuery<T> {
    /// Updates the amount of elements that can be present.
    pub fn per_page(self, per: i64) -> Self {
        PaginatedQuery {
            per_page: per,
            offset: (self.page - 1) * per,
            ..self
        }
    }

    pub fn perform<'a, U>(self, conn: &mut DbConnection) -> QueryResult<Paginated<U>>
    where
        Self: LoadQuery<'a, DbConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;

        Ok(Paginated {
            data: records,
            pages: total,
            total: total_pages,
            per_page,
        })
    }
}

impl<T: Query> Query for PaginatedQuery<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<DbConnection> for PaginatedQuery<T> {}

impl<T> QueryFragment<MultiBackend> for PaginatedQuery<T>
where
    T: QueryFragment<MultiBackend>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, MultiBackend>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub pages: i64,
    pub per_page: i64,
}
