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
use std::error::Error;

/// Represents a generic boxed [Error].
pub type BoxedError = Box<dyn Error + Send + 'static>;

/// Generic options container for the [search service][SearchService].
pub trait SearchOptions {
    fn allow_partial(&mut self, allow: bool) -> &mut Self;
    fn filter(&mut self, filter: Option<String>) -> &mut Self;
}

/// Abstraction on how we can do full-text search as easy as possible. You should
/// probably use charted's [search indexer](https://charts.noelware.org/docs/services/search-indexer)
/// to allow real-time indexing over objects as the API server will not perform indexing
/// once a entity has been inserted, deleted, or updated.
#[async_trait]
pub trait SearchService {
    type Options: SearchOptions;

    /// Performs any self-initialization and returns the result of the
    /// initialization, if necessary.
    async fn init(&self) -> Result<(), BoxedError> {
        Ok(())
    }

    async fn search_user(&self, query: String, options: Option<Self::Options>) -> Result<Option<User>, BoxedError>;
    async fn search_repo(
        &self,
        query: String,
        options: Option<Self::Options>,
    ) -> Result<Option<Repository>, BoxedError>;

    async fn search_organization(
        &self,
        query: String,
        options: Option<Self::Options>,
    ) -> Result<Option<Organization>, BoxedError>;
}
