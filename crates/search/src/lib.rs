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

use async_trait::async_trait;
use charted_common::models::entities::{Organization, Repository, User};
use eyre::Result;
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult<T: serde::ser::Serialize + Clone> {
    /// Offset.
    pub offset: usize,

    /// The limit
    pub limit: usize,

    /// The query itself.
    pub query: String,

    /// How long did it take to query.
    pub took: u64,

    /// All the hits based off the [query][SearchResult::query].
    pub hits: Vec<T>,
}

/// List of all indexes that must exist within a [`SearchService`].
pub static INDEXES: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        "charted-organizations".into(),
        "charted-repositories".into(),
        "charted-users".into(),
    ]
});

/// Generic options container for the [search service][SearchService].
pub trait SearchOptions {
    /// If the request allows partial data to be used instead of
    /// all data.
    ///
    /// ### Safety
    /// This method can panic if [`seal()`](SearchOptions::seal) was called.
    fn allow_partial(&mut self, allow: bool) -> &mut Self;

    /// Adds an additional filter.
    ///
    /// ### Safety
    /// This method can panic if [`seal()`](SearchOptions::seal) was called.
    fn filter<I: Into<String>>(&mut self, filter: I) -> &mut Self;

    /// Offset to use when querying objects.
    ///
    /// ### Safety
    /// This method can panic if [`seal()`](SearchOptions::seal) was called.
    fn offset(&mut self, offset: usize) -> &mut Self;

    /// Amount of hits to return.
    ///
    /// ### Safety
    /// This method can panic if [`seal()`](SearchOptions::seal) was called.
    fn limit(&mut self, limit: usize) -> &mut Self;

    /// Seals this mutable [`SearchOptions`] and returns an immutable,
    /// owned value to this [`SearchOptions`].
    fn seal(&mut self) -> Self;
}

/// Abstraction on how we can do full-text search as easy as possible. You should
/// probably use charted's [search indexer](https://charts.noelware.org/docs/services/search-indexer)
/// to allow real-time indexing over objects as the API server will not perform indexing
/// once a entity has been inserted, deleted, or updated.
#[async_trait]
pub trait SearchService {
    /// Options type for searching objects.
    type Options: SearchOptions + Send;

    /// Performs any self-initialization and returns the result of the
    /// initialization, if necessary.
    async fn init(&self) -> Result<()> {
        Ok(())
    }

    /// Performs the actual search and return an Option variant of `T`.
    async fn search<
        I: Into<String> + Debug + Send,
        Q: Into<String> + Debug + Send,
        T: serde::ser::Serialize + DeserializeOwned + Clone + Send + 'static,
    >(
        &self,
        index: I,
        query: Q,
        options: Self::Options,
    ) -> Result<SearchResult<T>>;

    /// Searches for an [`Organization`]. This shouldn't be implemented directly.
    async fn search_organizations<Q: Into<String> + Debug + Send>(
        &self,
        query: Q,
        options: Self::Options,
    ) -> Result<SearchResult<Organization>> {
        self.search::<&str, Q, Organization>("charted-organizations", query, options)
            .await
    }

    /// Searches for an [`Repository`]. This shouldn't be implemented directly.
    async fn search_repositories<Q: Into<String> + Debug + Send>(
        &self,
        query: Q,
        options: Self::Options,
    ) -> Result<SearchResult<Repository>> {
        self.search::<&str, Q, Repository>("charted-repositories", query, options)
            .await
    }

    /// Searches for an [`User`]. This shouldn't be implemented directly.
    async fn search_users<Q: Into<String> + Debug + Send>(
        &self,
        query: Q,
        options: Self::Options,
    ) -> Result<SearchResult<User>> {
        self.search::<&str, Q, User>("charted-users", query, options).await
    }
}
