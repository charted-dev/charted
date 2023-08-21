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

use bytes::Bytes;
use charted_common::{crypto::md5, COMMIT_HASH, VERSION};
use charted_storage::MultiStorageService;
use eyre::{Context, ContextCompat, Result};
use remi_core::StorageService;
use reqwest::Client;
use std::{fmt::Debug, fs::create_dir_all, path::Path};
use tracing::{debug, instrument, warn};

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon/";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar/";

/// Represents a module to implement user avatars that can be queried. charted-server
/// allows using Gravatar (with `users.gravatar_email`), or they can upload their own.
///
/// Uploading will have risks as in NSFW and other cases, but we plan to mitigate it.
#[derive(Debug, Clone)]
pub struct AvatarsModule {
    storage: MultiStorageService,
    client: Client,
}

impl AvatarsModule {
    /// Creates a new [`AvatarsModule`] with a [`MultiStorageService`] so that
    /// avatars can be queried and updated.
    pub fn new(storage: MultiStorageService) -> AvatarsModule {
        AvatarsModule {
            storage,
            client: Client::builder()
                .gzip(true)
                .user_agent(format!(
                    "Noelware/charted-server (+https://github.com/charted-dev/charted; v{VERSION}+{COMMIT_HASH})"
                ))
                .build()
                .unwrap(),
        }
    }

    /// Initializes this [`AvatarsModule`] by:
    ///
    /// * Creating the `./avatars` (where `./` relates to `config.storage.filesystem.directory`)
    pub async fn init(&self) -> Result<()> {
        if let MultiStorageService::Filesystem(fs) = self.storage.clone() {
            let avatars_path = fs.normalize("./avatars")?.context("unable to normalize [./avatars]")?;
            if !fs.exists(avatars_path.clone()).await? {
                warn!("directory [{}] doesn't exist! creating...", avatars_path.display());
                create_dir_all(avatars_path)?;
            }
        }

        Ok(())
    }

    /// Sends a request to Dicebear Avatars with the specified `email` to return
    /// a [`Bytes`] container of the avatar.
    #[instrument(name = "charted.avatars.identicons", skip(self))]
    pub async fn identicons(&self, id: u64) -> Result<Bytes> {
        let url = format!("{DICEBEAR_IDENTICONS_URI}/{id}.svg");
        debug!("now requesting to [{url}]",);

        self.client
            .get(url)
            .send()
            .await
            .context("unable to fulfill request")?
            .bytes()
            .await
            .context("unable to get raw bytes from request")
    }

    /// Sends a request to Gravatar with the specified `email` to return
    /// a [`Bytes`] container of the avatar.
    #[instrument(name = "charted.avatars.identicons", skip(self))]
    pub async fn gravatar(&self, email: String) -> Result<Bytes> {
        let hash = md5(email);
        let url = format!("{GRAVATAR_URI}/{hash}.png");
        debug!("requesting to [{url}]");

        self.client
            .get(url)
            .send()
            .await
            .context("unable to fulfill request")?
            .bytes()
            .await
            .context("unable to get raw bytes from request")
    }

    #[instrument(name = "charted.avatars.query", skip(self, _failed))]
    async fn query<I: AsRef<Path> + Debug + Send, F>(&self, path: I, _failed: F) -> Result<Option<Bytes>>
    where
        F: FnOnce(),
    {
        let Some(_bytes) = self.storage.open(path).await? else {
            return Ok(None);
        };

        Ok(None)
    }
}
