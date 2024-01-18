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

use eyre::Context;
use md5::Md5;
use noelware_remi::StorageService;
use remi::{Bytes, StorageService as _, UploadRequest};
use remi_fs::default_resolver;
use reqwest::{Client, StatusCode};
use std::fs::create_dir_all;

use crate::rand_string;

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";
const ACCEPTABLE_CONTENT_TYPE: &[&str] = &["image/png", "image/jpeg", "image/gif", "image/svg"];

/// Represents a module to implement user avatars that can be queried. charted-server
/// allows using Gravatar (with `users.gravatar_email`), or they can upload their own.
///
/// Uploading will have risks as in NSFW and other cases, but we plan to mitigate it.
#[derive(Clone)]
pub struct AvatarsModule<'s> {
    storage: &'s StorageService,
    client: Client,
}

impl<'s> AvatarsModule<'s> {
    /// Creates a new [`AvatarsModule`] instance.
    pub fn new(storage: &'s StorageService) -> AvatarsModule {
        AvatarsModule {
            storage,
            client: Client::builder()
                .user_agent(format!(
                    "Noelware/charted-server (+https://github.com/charted-dev/charted; v{})",
                    crate::version(),
                ))
                .build()
                .unwrap(),
        }
    }

    /// Initializes this [`AvatarsModule`] by creating the necessary directories
    /// if you're using the local filesystem.
    pub async fn init(&self) -> eyre::Result<()> {
        if let StorageService::Filesystem(fs) = self.storage {
            for path in [
                fs.normalize("./avatars/organizations")?.unwrap(),
                fs.normalize("./icons/repositories")?.unwrap(),
                fs.normalize("./avatars/users")?.unwrap(),
            ] {
                if !path.try_exists()? {
                    warn!(dir = %path.display(), "directory doesn't exist");
                    create_dir_all(path)?;
                }
            }
        }

        Ok(())
    }

    /// Sends a request to Dicebear Avatars with a user ID to return a [`Bytes`] container
    /// of the avatar itself.
    #[instrument(name = "charted.avatars.identicons", skip(self))]
    pub async fn identicons(&self, id: u64) -> eyre::Result<Bytes> {
        let url = format!("{DICEBEAR_IDENTICONS_URI}/{id}.png");
        debug!(%url, "sending request");

        self.client
            .get(url)
            .send()
            .await
            .context("unable to fulfill request :(")?
            .bytes()
            .await
            .context("unable to get raw bytes from request")
    }

    /// Sends a request to Gravatar with the specified `email` to return
    /// a [`Bytes`] container of the avatar.
    #[instrument(name = "charted.avatars.gravatar", skip_all)]
    pub async fn gravatar<E: Into<String>>(&self, email: E) -> eyre::Result<Option<Bytes>> {
        let email = email.into();
        let hash = {
            use md5::{digest::FixedOutput, Digest};

            let mut hasher: md5::Md5 = Md5::new();
            Digest::update(&mut hasher, <String as AsRef<[u8]>>::as_ref(&email));

            hex::encode(hasher.finalize_fixed().as_slice())
        };

        let url = format!("{GRAVATAR_URI}/{hash}.png");
        debug!(%url, "sending request");

        let res = self
            .client
            .get(url)
            .send()
            .await
            .context("unable to fulfill request :(")?;

        if res.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        res.bytes().await.context("unable to get bytes from request").map(Some)
    }

    #[instrument(name = "charted.avatars.get", skip(self, entity, hash))]
    async fn entity<E: AsRef<str>, I: Into<String>>(
        &self,
        entity: E,
        id: u64,
        hash: Option<I>,
    ) -> eyre::Result<Option<Bytes>> {
        let entity = entity.as_ref();
        let hash = match hash {
            Some(i) => i.into(),
            None => String::from("current.png"),
        };

        self.storage
            .open(format!("./avatars/{}/{id}/{hash}", entity))
            .await
            .context(format!("unable to open file [./avatars/{}/{id}/{hash}", entity))
    }

    #[instrument(name = "charted.avatars.upload", skip(self, entity, data))]
    async fn upload<E: AsRef<str>>(&self, entity: E, id: u64, data: Bytes) -> eyre::Result<String> {
        let entity = entity.as_ref();
        let content_type = default_resolver(&data);

        let mut already_found = false;
        for ct in ACCEPTABLE_CONTENT_TYPE.iter() {
            if content_type.starts_with(ct) {
                already_found = true;
                break;
            }
        }

        if !already_found {
            return Err(eyre!(
                "invalid content type [{content_type}], expected [image/png, image/jpeg, image/gif, image/svg]"
            ));
        }

        let request = UploadRequest::default()
            .with_content_type(Some(&content_type))
            .with_data(data);

        let hash = rand_string(4);
        self.storage
            .upload(
                format!(
                    "./avatars/{entity}/{id}/{hash}.{}",
                    match content_type.as_str() {
                        "image/png" => "png",
                        "image/jpeg" => "jpg",
                        "image/gif" => "gif",
                        "image/svg" => "svg",
                        _ => unreachable!(),
                    }
                ),
                request,
            )
            .await
            .map(|()| hash)
            .context("unable to upload avatar")
    }

    /// Retrieve a user's avatar and returns the [`Bytes`] of it.
    pub async fn user<H: Into<String>>(&self, uid: u64, hash: Option<H>) -> eyre::Result<Option<Bytes>> {
        self.entity("users", uid, hash).await
    }

    /// Retrieve a repository icon and returns the [`Bytes`] of it.
    pub async fn repository<H: Into<String>>(&self, uid: u64, hash: Option<H>) -> eyre::Result<Option<Bytes>> {
        self.entity("repositories", uid, hash).await
    }

    /// Retrieve a organization's avatar and returns the [`Bytes`] of it.
    pub async fn organization<H: Into<String>>(&self, uid: u64, hash: Option<H>) -> eyre::Result<Option<Bytes>> {
        self.entity("organizations", uid, hash).await
    }

    /// Uploads a user's avatar
    pub async fn upload_user_avatar(&self, id: u64, data: Bytes) -> eyre::Result<String> {
        self.upload("users", id, data).await
    }

    /// Uploads a organization's avatar
    pub async fn upload_organization_avatar(&self, id: u64, data: Bytes) -> eyre::Result<String> {
        self.upload("organizations", id, data).await
    }

    /// Uploads a repository icon
    pub async fn upload_repo_icon(&self, id: u64, data: Bytes) -> eyre::Result<String> {
        self.upload("repositories", id, data).await
    }
}
