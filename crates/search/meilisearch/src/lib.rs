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
use charted_config::search::meilisearch::Config;
use charted_search::{Backend, Indexable};
use meilisearch_sdk::{FailedTask, Task, TaskInfo};
use std::borrow::Cow;
use tracing::{error, trace};

/// Represents a [`Backend`] that allows to search through objects with [Meilisearch](https://meilisearch.com).
#[derive(Debug, Clone)]
pub struct Meilisearch(meilisearch_sdk::Client);

impl Meilisearch {
    /// Creates a new [`Meilisearch`] object with the [configuration reference][Config]
    pub fn new(config: &Config) -> Meilisearch {
        let client = meilisearch_sdk::Client::new(&config.host, config.master_key.as_ref());
        Meilisearch(client)
    }

    /// Returns a reference to the constructed Meilisearch SDK client.
    pub fn client(&self) -> &meilisearch_sdk::Client {
        &self.0
    }
}

#[async_trait]
impl Backend for Meilisearch {
    async fn search(
        &self,
        index: Cow<'static, str>,
        query: Cow<'static, str>,
    ) -> eyre::Result<&(dyn erased_serde::Serialize + Send + Sync)> {
        trace!(%index, %query, "querying Meilisearch...");

        todo!()
    }

    async fn delete(&self, obj: &(dyn Indexable + Send + Sync)) -> eyre::Result<()> {
        trace!(index = obj.index(), id = obj.id(), "deleting object");
        let index = self.client().index(obj.index());
        let task = index.delete_documents(&[obj.id()]).await?;

        process_task(self.client(), &task, obj).await
    }

    async fn index(&self, obj: &(dyn Indexable + Send + Sync)) -> eyre::Result<()> {
        trace!(index = obj.index(), id = obj.id(), "indexing object...");
        let client = self.client();
        let index = client.index(obj.index());

        let task = index.add_or_replace(&[obj], Some(obj.id_field())).await?;
        process_task(client, &task, obj).await
    }
}

async fn process_task(
    client: &meilisearch_sdk::Client,
    task: &TaskInfo,
    obj: &(dyn Indexable + Send + Sync),
) -> eyre::Result<()> {
    trace!(
        index = obj.index(),
        task.id = task.task_uid,
        task.status,
        "task for indexing a object has been reported, waiting until task is completed or failed"
    );

    client
        .wait_for_task(&task, None, None)
        .await
        .map(|tsk| {
            let status = match tsk {
                Task::Enqueued { .. } => "enqueued",
                Task::Processing { .. } => "processing...",
                Task::Failed { .. } => "failed",
                Task::Succeeded { .. } => "succeeded",
            };

            let error = match tsk {
                Task::Failed {
                    content: FailedTask { error, .. },
                } => {
                    sentry::capture_error(&error);
                    Some(error)
                }

                _ => None,
            };

            trace!(
                index = obj.index(),
                id = obj.id(),
                task.id = task.task_uid,
                status,
                ?error,
                "task has given a status"
            );
        })
        .inspect_err(|err| {
            error!(%err, index = obj.index(), task.id = task.task_uid, "failed to index object due to");
            sentry::capture_error(err);
        })
        .map_err(eyre::Report::from)
}
