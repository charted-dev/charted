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
//
//! # 🐻‍❄️📦 `charted_datastore`
//! This crate is a internal crate that defines a [`DataStore`] that wraps around
//! a [`StorageService`](azalia_remi::core::StorageService) to handle namespacing
//! through objects.
//!
//! Through the lifecycle of charted, we have always let loose on how data is stored
//! in persistent storage and this crate helps mitigate mistakes.
//!
//! **Namespaces** are a concept that allows to isolate calls to a [`StorageService`](azalia_remi::core::StorageService)
//! with a specified prefix. For example, if we need to access Noel's chart index,
//! we can build a [`Namespace`]:
//!
//! ```no_run
//! use charted_datastore::{fs, DataStore, remi::StorageService as _};
//! use charted_config::storage::Config;
//!
//! # let _ = tokio_test::block_on(async {
//! // Builds a `DataStore` that is fully initialized.
//! let ds = DataStore::new(&Config::Filesystem(fs::StorageConfig {
//!     directory: "./data".into(),
//! })).await?;
//!
//! // next, get a namespace. namespaces aren't cached since it's just a pointer
//! // to the storage service that `ds` has control over and the name of it.
//! let ns = ds.namespace("metadata");
//!
//! // since `Namespace` implements `remi::StorageService`, we can call
//! // operations and we can ensure that `./metadata/noel/index.yaml`
//! // is called.
//! ns.open("noel/index.yaml").await?;
//! #
//! # Ok::<(), eyre::Report>(())
//! # });
//! ```

// Export the main `remi` crate.
use azalia_remi::{
    StorageService,
    core::{Blob, UploadRequest},
};
pub use azalia_remi::{azure, core as remi, fs, s3};
use charted_config::storage;
use charted_core::ResultExt;
use eyre::Context;
use remi::StorageService as _;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    ops::Deref,
    path::{Path, PathBuf},
};
use tracing::debug;

#[derive(Clone)]
pub struct DataStore(azalia_remi::StorageService);
impl DataStore {
    /// Creates a new [`DataStore`] instance and initializes the datastore itself.
    pub async fn new(config: &storage::Config) -> eyre::Result<Self> {
        let service = match config {
            storage::Config::Filesystem(fs) => {
                StorageService::Filesystem(fs::StorageService::new(fs.directory.clone()))
            }

            storage::Config::S3(s3) => StorageService::S3(s3::StorageService::new(s3.to_owned())),
            storage::Config::Azure(azure) => StorageService::Azure(
                azure::StorageService::new(azure.to_owned()).context("failed to build Azure datastore")?,
            ),
        };

        remi::StorageService::init(&service).await.into_report()?;
        Ok(Self(service))
    }

    /// Return a [`Namespace`] object that allows to group persisted data to make it
    /// easier to handle the datastore itself.
    pub fn namespace<'storage, N: Into<Cow<'storage, str>>>(&'storage self, namespace: N) -> Namespace<'storage> {
        Namespace {
            handle: &self.0,
            namespace: namespace.into(),
        }
    }

    /// Returns `true` if the datastore is a local filesystem path.
    pub const fn is_filesystem(&self) -> bool {
        matches!(self, DataStore(StorageService::Filesystem(_)))
    }
}

impl Deref for DataStore {
    type Target = azalia_remi::StorageService;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A namespace is a group of persisted data. This implements [`remi::StorageService`]
/// so you can do operations and the namespace will be appended in the path.
#[derive(Clone)]
pub struct Namespace<'storage> {
    handle: &'storage StorageService,
    namespace: Cow<'storage, str>,
}

impl Debug for Namespace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Namespace").field(&self.namespace).finish()
    }
}

impl Display for Namespace<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "namespace {} ({})", self.namespace, self.handle.name())
    }
}

#[remi::async_trait]
impl<'storage> remi::StorageService for Namespace<'storage> {
    type Error = azalia_remi::Error;

    fn name(&self) -> Cow<'static, str>
    where
        Self: Sized,
    {
        Cow::Owned(format!("{} (namespace {})", self.handle.name(), self.namespace))
    }

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<remi::Bytes>, Self::Error> {
        async fn _intercept(me: &Namespace<'_>, path: &Path) -> Result<Option<remi::Bytes>, azalia_remi::Error> {
            let new_path = match me.handle {
                StorageService::Filesystem(_) => PathBuf::from(format!("./{}", me.namespace)).join(path),
                StorageService::Azure(_) | StorageService::S3(_) => {
                    PathBuf::from(me.namespace.to_string()).join(path)
                }

                _ => unreachable!(),
            };

            debug!(path = %new_path.display(), "intercept :: datastore[{}]: open", me.handle.name());
            me.handle.open(new_path).await
        }

        _intercept(self, path.as_ref()).await
    }

    async fn blob<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<Blob>, Self::Error> {
        async fn _intercept(me: &Namespace<'_>, path: &Path) -> Result<Option<Blob>, azalia_remi::Error> {
            let new_path = match me.handle {
                StorageService::Filesystem(_) => PathBuf::from(format!("./{}", me.namespace)).join(path),
                StorageService::Azure(_) | StorageService::S3(_) => {
                    PathBuf::from(me.namespace.to_string()).join(path)
                }

                _ => unreachable!(),
            };

            debug!(path = %new_path.display(), "intercept :: datastore[{}]: blob", me.handle.name());
            me.handle.blob(new_path).await
        }

        _intercept(self, path.as_ref()).await
    }

    async fn blobs<P: AsRef<Path> + Send>(
        &self,
        path: Option<P>,
        options: Option<remi::ListBlobsRequest>,
    ) -> Result<Vec<Blob>, Self::Error> {
        let mut new_path = match self.handle {
            StorageService::Filesystem(_) => PathBuf::from(format!("./{}", self.namespace)),
            StorageService::Azure(_) | StorageService::S3(_) => PathBuf::from(self.namespace.to_string()),

            _ => unreachable!(),
        };

        if let Some(p) = path {
            new_path = new_path.join(p.as_ref());
        }

        debug!(path = %new_path.display(), "intercept :: datastore[{}]: blobs", self.handle.name());
        self.handle.blobs(Some(new_path), options).await
    }

    async fn delete<P: AsRef<Path> + Send>(&self, path: P) -> Result<(), Self::Error> {
        async fn _intercept(me: &Namespace<'_>, path: &Path) -> Result<(), azalia_remi::Error> {
            let new_path = match me.handle {
                StorageService::Filesystem(_) => PathBuf::from(format!("./{}", me.namespace)).join(path),
                StorageService::Azure(_) | StorageService::S3(_) => {
                    PathBuf::from(me.namespace.to_string()).join(path)
                }

                _ => unreachable!(),
            };

            debug!(path = %new_path.display(), "intercept :: datastore[{}]: delete", me.handle.name());
            me.handle.delete(new_path).await
        }

        _intercept(self, path.as_ref()).await
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> Result<bool, Self::Error> {
        async fn _intercept(me: &Namespace<'_>, path: &Path) -> Result<bool, azalia_remi::Error> {
            let new_path = match me.handle {
                StorageService::Filesystem(_) => PathBuf::from(format!("./{}", me.namespace)).join(path),
                StorageService::Azure(_) | StorageService::S3(_) => {
                    PathBuf::from(me.namespace.to_string()).join(path)
                }

                _ => unreachable!(),
            };

            debug!(path = %new_path.display(), "intercept :: datastore[{}]: exists", me.handle.name());
            me.handle.exists(new_path).await
        }

        _intercept(self, path.as_ref()).await
    }

    async fn upload<P: AsRef<Path> + Send>(&self, path: P, request: UploadRequest) -> Result<(), Self::Error> {
        async fn _intercept(
            me: &Namespace<'_>,
            path: &Path,
            request: UploadRequest,
        ) -> Result<(), azalia_remi::Error> {
            let new_path = match me.handle {
                StorageService::Filesystem(_) => PathBuf::from(format!("./{}", me.namespace)).join(path),
                StorageService::Azure(_) | StorageService::S3(_) => {
                    PathBuf::from(me.namespace.to_string()).join(path)
                }

                _ => unreachable!(),
            };

            debug!(path = %new_path.display(), "intercept :: datastore[{}]: upload", me.handle.name());
            me.handle.upload(new_path, request).await
        }

        _intercept(self, path.as_ref(), request).await
    }
}
