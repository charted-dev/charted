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
use tracing::{error, info, trace};

// dummy indexable object for creating an Meilisearch index with [`process_task`].
#[derive(serde::Serialize)]
struct DummyIndexable(String);
impl Indexable for DummyIndexable {
    fn id(&self) -> i64 {
        i64::default()
    }

    fn index<'a>(&self) -> Cow<'a, str> {
        Cow::Owned(self.0.clone())
    }
}

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

    /// Creates an `index` with a `primary_key` and polls to completion when the task has *either* failed
    /// or succeeded.
    pub async fn create_index_if_not_exists<'a, I: AsRef<str> + 'static>(
        &self,
        index: I,
        primary_key: Option<&'a str>,
    ) -> eyre::Result<()> {
        let index = index.as_ref();
        info!(index, primaryKey = ?primary_key, "creating index with primary key");

        let client = self.client();
        process_task(
            client,
            &client.create_index(index, primary_key).await?,
            &DummyIndexable(index.to_owned()),
        )
        .await
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
        trace!(index = %obj.index(), id = obj.id(), "deleting object");
        let index = self.client().index(obj.index());
        let task = index.delete_documents(&[obj.id()]).await?;

        process_task(self.client(), &task, obj).await
    }

    async fn index(&self, obj: &(dyn Indexable + Send + Sync)) -> eyre::Result<()> {
        let idx = obj.index();

        trace!(index = %obj.index(), id = obj.id(), "indexing object...");
        let client = self.client();
        let index = client.index(idx.clone());

        self.create_index_if_not_exists(idx, Some(&obj.id_field())).await?;

        let task = index.add_or_replace(&[obj], Some(&obj.id_field())).await?;
        process_task(client, &task, obj).await
    }
}

async fn process_task(
    client: &meilisearch_sdk::Client,
    task: &TaskInfo,
    obj: &(dyn Indexable + Send + Sync),
) -> eyre::Result<()> {
    trace!(
        index = %obj.index(),
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
                index = %obj.index(),
                id = obj.id(),
                task.id = task.task_uid,
                status,
                ?error,
                "task has given a status"
            );
        })
        .inspect_err(|err| {
            error!(%err, index = %obj.index(), task.id = task.task_uid, "failed to index object due to");
            sentry::capture_error(err);
        })
        .map_err(eyre::Report::from)
}

#[cfg(tests_that_are_excluded_hehdotdotdotdot)]
mod tests {
    use bollard::Docker;
    use charted_search::Backend as _;
    use eyre::Context;
    use testcontainers::runners::AsyncRunner;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    /// List of Meilisearch versions that should be tested for.
    const MEILISEARCH_VERSIONS: &[&str] = &["latest", "v1.7", "v1.6", "v1.5"];

    macro_rules! build_testcases {
        (
            $(
                fn $name:ident($config:ident) $code:block
            )*
        ) => {
            $(
                #[tokio::test]
                async fn $name() -> ::eyre::Result<()> {
                    let _guard = tracing_subscriber::registry()
                        .with(tracing_subscriber::fmt::layer())
                        .set_default();

                    match try_ping().await {
                        Ok(true) => {}
                        Ok(_) => unreachable!(),
                        Err(e) => {
                            eprintln!("failed to connect to Docker daemon: {e:?}");
                            return Err(e);
                        }
                    }

                    for version in MEILISEARCH_VERSIONS {
                        println!("...spinning up Meilisearch container with tag [getmeili/meilisearch:{version}]");

                        let image = ::testcontainers::GenericImage::new("getmeili/meilisearch", version).with_exposed_port(7700);
                        let container: ::testcontainers::RunnableImage<::testcontainers::GenericImage> = (image, vec![]).into();
                        let container = container.start().await;
                        let (host, port) = (container.get_host().await, container.get_host_port_ipv4(7700).await);

                        let $config = ::charted_config::search::meilisearch::Config {
                            master_key: None,
                            host: String::from(format!("http://{host}:{port}"))
                        };

                        let __ret: ::eyre::Result<()> = $code;
                        __ret.with_context(|| format!("failed to run test {} with container [getmeili/meilisearch:{version}]", stringify!($name)))?;
                    }

                    Ok(())
                }
            )*
        };
    }

    build_testcases! {
        fn test_indexing_objects(config) {
            let backend = super::Meilisearch::new(&config);

            // test indexing
            let noel = charted_entities::User {
                username: "noel".parse()?,
                admin: true,
                id: 1,

                ..Default::default()
            };

            let res = backend.index(&noel).await;
            assert!(matches!(res, Ok(())));

            Ok(())
        }
    }

    async fn try_ping() -> eyre::Result<bool> {
        Docker::connect_with_defaults()?
            .ping()
            .await
            .map(|_| true)
            .context("cannot ping Docker daemon?!")
    }
}
