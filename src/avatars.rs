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

use crate::HTTP_CLIENT;
use charted_common::rand_string;
use eyre::Context;
use md5::Md5;
use noelware_remi::StorageService;
use remi::{Bytes, StorageService as _, UploadRequest};
use remi_fs::default_resolver;
use reqwest::StatusCode;
use std::fs::create_dir_all;

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";
const ACCEPTABLE_CONTENT_TYPE: &[&str] = &["image/png", "image/jpeg", "image/gif", "image/svg"];

/// Initializes the data storage for all user/organization avatars and repository icons.
pub async fn init(storage: &StorageService) -> eyre::Result<()> {
    if let StorageService::Filesystem(fs) = storage {
        info!("initializing data storage for avatars/icons");
        for path in [
            fs.normalize("./avatars/organizations")?.unwrap(),
            fs.normalize("./avatars/repositories")?.unwrap(),
            fs.normalize("./avatars/users")?.unwrap(),
        ] {
            if !path.try_exists()? {
                warn!(dir = %path.display(), "directory doesn't exist, now creating");
                create_dir_all(path)?;
            }
        }
    }

    Ok(())
}

/// Sends a request to Dicebear Avatars with a entity ID of a PNG image, which is the
/// default avatar that people use.
#[instrument(name = "charted.avatars.identicons")]
pub async fn identicons(id: u64) -> eyre::Result<Bytes> {
    let url = format!("{DICEBEAR_IDENTICONS_URI}/{id}.png");
    debug!(%url, "sending request");

    HTTP_CLIENT
        .get(url)
        .send()
        .await
        .context("unable to fulfill http request")?
        .bytes()
        .await
        .context("was unable to get raw content from request")
}

/// Sends a request to Gravatar with the specified `email` to return
/// a [`Bytes`] container of the avatar.
#[instrument(name = "charted.avatars.gravatar", skip_all)]
pub async fn gravatar<E: Into<String>>(email: E) -> eyre::Result<Option<Bytes>> {
    let email = email.into();
    let hash = {
        use md5::{digest::FixedOutput, Digest};

        let mut hasher: md5::Md5 = Md5::new();
        Digest::update(&mut hasher, <String as AsRef<[u8]>>::as_ref(&email));

        hex::encode(hasher.finalize_fixed().as_slice())
    };

    let url = format!("{GRAVATAR_URI}/{hash}.png");
    debug!(%url, "sending request");

    let res = HTTP_CLIENT
        .get(url)
        .send()
        .await
        .context("unable to fulfill request :(")?;

    if res.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    res.bytes().await.context("unable to get bytes from request").map(Some)
}

/// Retrieve a user's avatar and returns the [`Bytes`] of it.
pub async fn user<H: Into<String>>(storage: &StorageService, uid: u64, hash: Option<H>) -> eyre::Result<Option<Bytes>> {
    entity(storage, "users", "avatars", uid, hash).await
}

/// Retrieve a repository icon and returns the [`Bytes`] of it.
pub async fn repository<H: Into<String>>(
    storage: &StorageService,
    uid: u64,
    hash: Option<H>,
) -> eyre::Result<Option<Bytes>> {
    entity(storage, "repositories", "icons", uid, hash).await
}

/// Retrieve a organization's avatar and returns the [`Bytes`] of it.
pub async fn organization<H: Into<String>>(
    storage: &StorageService,
    uid: u64,
    hash: Option<H>,
) -> eyre::Result<Option<Bytes>> {
    entity(storage, "organizations", "avatars", uid, hash).await
}

/// Uploads a user's avatar
pub async fn upload_user_avatar(storage: &StorageService, id: u64, data: Bytes) -> eyre::Result<String> {
    upload(storage, "users", "avatars", id, data).await
}

/// Uploads a organization's avatar
pub async fn upload_organization_avatar(storage: &StorageService, id: u64, data: Bytes) -> eyre::Result<String> {
    upload(storage, "organizations", "avatars", id, data).await
}

/// Uploads a repository icon
pub async fn upload_repo_icon(storage: &StorageService, id: u64, data: Bytes) -> eyre::Result<String> {
    upload(storage, "repositories", "icons", id, data).await
}

#[instrument(name = "charted.avatars.get", skip_all)]
async fn entity<E: AsRef<str>, I: Into<String>>(
    storage: &StorageService,
    entity: E,
    ty: impl AsRef<str>,
    id: u64,
    hash: Option<I>,
) -> eyre::Result<Option<Bytes>> {
    let entity = entity.as_ref();
    let ty = ty.as_ref();
    let hash = match hash {
        Some(i) => i.into(),
        None => String::from("current.png"),
    };

    storage
        .open(format!("./{ty}/{entity}/{id}/{hash}"))
        .await
        .context(format!("unable to open file [./avatars/{}/{id}/{hash}", entity))
}

#[instrument(name = "charted.avatars.upload", skip_all)]
async fn upload<E: AsRef<str>>(
    storage: &StorageService,
    entity: E,
    ty: impl AsRef<str>,
    id: u64,
    data: Bytes,
) -> eyre::Result<String> {
    let entity = entity.as_ref();
    let ty = ty.as_ref();
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
    storage
        .upload(
            format!(
                "./{ty}/{entity}/{id}/{hash}.{}",
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
