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
use charted_common::{crypto::md5, rand_string, COMMIT_HASH, VERSION};
use charted_storage::MultiStorageService;
use eyre::{Context, Result};
use remi_core::{StorageService, UploadRequest};
use reqwest::{Client, StatusCode};
use std::{fmt::Debug, fs::create_dir_all};
use tracing::{debug, instrument, warn};

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";

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
            let others = vec![
                fs.normalize("./avatars/organizations")?.unwrap(),
                fs.normalize("./avatars/repositories")?.unwrap(),
                fs.normalize("./avatars/users")?.unwrap(),
            ];

            for p in others.iter() {
                if !p.exists() {
                    warn!("directory [{}] doesn't exist! creating...", p.display());
                    create_dir_all(p)?;
                }
            }
        }

        Ok(())
    }

    /// Sends a request to Dicebear Avatars with the specified `email` to return
    /// a [`Bytes`] container of the avatar.
    #[instrument(name = "charted.avatars.identicons", skip(self))]
    pub async fn identicons(&self, id: u64) -> Result<Bytes> {
        let url = format!("{DICEBEAR_IDENTICONS_URI}/{id}.png");
        debug!("now requesting to [{url}]");

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
    #[instrument(name = "charted.avatars.gravatar", skip(self))]
    pub async fn gravatar(&self, email: String) -> Result<Option<Bytes>> {
        let hash = md5(email);
        let url = format!("{GRAVATAR_URI}/{hash}.png");
        debug!("requesting to [{url}]");

        let res = self.client.get(url).send().await.context("unable to fulfill request")?;
        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        res.bytes().await.context("unable to get bytes from request").map(Some)
    }

    /// Retrieve a user's avatar and returns the [`Bytes`] of it.
    #[instrument(name = "charted.avatars.user", skip(self))]
    pub async fn user(&self, uid: u64, hash: String) -> Result<Option<Bytes>> {
        self.storage
            .open(format!("./avatars/users/{uid}/{hash}"))
            .await
            .context("unable to open [./avatars/{uid}/{hash}]")
    }

    #[instrument(name = "charted.avatars.repository", skip(self))]
    pub async fn repository(&self, id: u64, hash: String) -> Result<Option<Bytes>> {
        self.storage
            .open(format!("./avatars/repositories/{id}/{hash}"))
            .await
            .context("unable to open [./avatars/repositories/{id}/{hash}]")
    }

    #[instrument(name = "charted.avatars.organization", skip(self))]
    pub async fn organization(&self, id: u64, hash: String) -> Result<Option<Bytes>> {
        self.storage
            .open(format!("./avatars/organizations/{id}/{hash}"))
            .await
            .context("unable to open [./avatars/organizations/{id}/{hash}]")
    }

    #[instrument(name = "charted.avatars.user.upload", skip(self, data))]
    pub async fn upload_user_avatar(&self, id: u64, data: Bytes, ct: String, ext: &str) -> Result<String> {
        let hash = format!("{}.{ext}", rand_string(4));
        let request = UploadRequest::default()
            .with_content_type(Some(ct))
            .with_data(data)
            .seal();

        self.storage
            .upload(format!("./avatars/users/{id}/{hash}"), request)
            .await
            .map(|_| hash)
            .context("unable to upload user avatar")
    }

    #[instrument(name = "charted.avatars.organization.upload", skip(self, data))]
    pub async fn upload_org_avatar(&self, id: u64, data: Bytes, ct: String, ext: &str) -> Result<String> {
        let hash = format!("{}.{ext}", rand_string(4));
        let request = UploadRequest::default()
            .with_content_type(Some(ct))
            .with_data(data)
            .seal();

        self.storage
            .upload(format!("./avatars/users/{id}/{hash}"), request)
            .await
            .map(|_| hash)
            .context("unable to upload organization avatar")
    }

    #[instrument(name = "charted.avatars.repository.upload", skip(self, data))]
    pub async fn upload_repo_icon(&self, id: u64, data: Bytes, ct: String, ext: &str) -> Result<String> {
        let hash = format!("{}.{ext}", rand_string(4));
        let request = UploadRequest::default()
            .with_content_type(Some(ct))
            .with_data(data)
            .seal();

        self.storage
            .upload(format!("./avatars/repositories/{id}/{hash}"), request)
            .await
            .map(|_| hash)
            .context("unable to upload repository icon")
    }
}
