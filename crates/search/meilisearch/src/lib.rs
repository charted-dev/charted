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

use async_trait::async_trait;
use charted_config::search::SearchConfig;
use charted_search::{SearchResult, SearchService, INDEXES};
use eyre::{eyre, Result};
use meilisearch_sdk::{search::SearchQuery, Client};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tracing::{debug, info, warn};

/// Represents the search options ofr Meilisearch.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    offset: usize,
    limit: usize,
}

#[derive(Debug, Clone)]
pub struct MeilisearchSearchService {
    client: Client,
}

impl MeilisearchSearchService {
    pub fn new(config: charted_config::Config) -> Result<MeilisearchSearchService> {
        if let Some(SearchConfig::Meilisearch(meili)) = config.search.clone() {
            let api_key = meili.master_key()?;
            let client = Client::new(meili.server_url.clone(), api_key);

            return Ok(MeilisearchSearchService { client });
        }

        Err(eyre!("configured search service was not Meilisearch"))
    }
}

#[async_trait]
impl SearchService for MeilisearchSearchService {
    type Options = SearchOptions;

    async fn init(&self) -> Result<()> {
        info!("performing initialization...");

        let indexes = self.client.get_indexes().await.map(|res| res.results)?;
        for index in INDEXES.iter() {
            debug!("checking if index [{index}] exists...");

            if !indexes.iter().any(|f| &f.uid == index) {
                warn!("index [{index}] doesn't exist, creating!");
                let task = self.client.create_index(index, Some("id")).await?;

                info!("waiting for task [{}] to complete", task.task_uid);
                task.wait_for_completion(&self.client, None, None).await?;
            }
        }

        Ok(())
    }

    async fn search<
        I: Into<String> + Debug + Send,
        Q: Into<String> + Debug + Send,
        T: serde::ser::Serialize + DeserializeOwned + Clone + Send + 'static,
    >(
        &self,
        index: I,
        query: Q,
        options: Self::Options,
    ) -> Result<SearchResult<T>> {
        let index = self.client.index(index);
        let query: String = query.into();
        let mut opts = SearchQuery::new(&index);

        opts.offset = Some(options.offset);
        opts.query = Some(&query);
        opts.limit = Some(options.limit);

        let result = index.execute_query::<T>(&opts).await?;
        Ok(SearchResult {
            offset: result.offset.unwrap_or(options.offset),
            limit: result.limit.unwrap_or(options.limit),
            query,
            took: result.processing_time_ms as u64,
            hits: result.hits.iter().map(|res| res.result.clone()).collect(),
        })
    }
}
