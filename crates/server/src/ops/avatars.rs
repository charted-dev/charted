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

//! The `ops::avatars` module contains common operations that operate for user & organization avatars,
//! and repository icons.

use crate::ServerContext;
use axum::body::Bytes;
use charted_common::rand_string;
use eyre::{eyre, Context};
use md5::{digest::FixedOutput, Digest};
use noelware_remi::{fs::default_resolver, StorageService};
use once_cell::sync::OnceCell;
use remi::{StorageService as _, UploadRequest};
use reqwest::StatusCode;
use std::{fs::create_dir_all, future::Future};
use tracing::{instrument, trace, warn};

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";
const ACCEPTABLE_CONTENT_TYPE: &[&str] = &["image/png", "image/jpeg", "image/gif"];

fn init(storage: &StorageService) -> eyre::Result<()> {
    // As of 28/06/24, `OnceLock::try_get_or_init` is unstable because of issue[ref]. We
    // could go with a nightly compiler but that requires work and we want to build charted
    // with a stable compiler because why not.
    //
    // For now, we will use `OnceCell` from `once_cell` and once the issue[ref] is stablised,
    // we can *potentially* remove the `once_cell` dependency.
    //
    // ref: https://github.com/rust-lang/rust/issues/109737
    static ONCE: OnceCell<()> = OnceCell::new();
    ONCE.get_or_try_init(move || {
        let storage = storage.clone();
        trace!("initializing data storage for avatars");

        if let StorageService::Filesystem(ref fs) = storage {
            for path in [
                fs.normalize("./avatars/organizations")?.unwrap(),
                fs.normalize("./avatars/users")?.unwrap(),
            ] {
                if !path.try_exists()? {
                    warn!(dir = %path.display(), "directory doesn't exist! creating");
                    create_dir_all(path)?;
                }
            }
        }

        Ok(())
    })
    .copied()
}

#[instrument(name = "charted.ops.avatars.identicons", skip(ctx))]
pub async fn identicons(ctx: &ServerContext, id: u64) -> eyre::Result<Bytes> {
    init(&ctx.storage)?;

    ctx.http
        .get(format!("{DICEBEAR_IDENTICONS_URI}/{id}.png"))
        .send()
        .await
        .context("unable to fulfill HTTP request")?
        .bytes()
        .await
        .context("failed to deserialize content")
}

#[instrument(name = "charted.ops.avatars.gravatar", skip_all)]
pub async fn gravatar<E: Into<String>>(ctx: &ServerContext, email: E) -> eyre::Result<Option<Bytes>> {
    init(&ctx.storage)?;

    let email = email.into();
    let hash = compute_md5_hash(email.as_ref());
    let url = format!("{GRAVATAR_URI}/{hash}.png");

    let res = ctx
        .http
        .get(url)
        .send()
        .await
        .context("unable to fulfill HTTP request")?;

    if res.status() == StatusCode::NOT_FOUND {
        return Ok(None);
    }

    let res = match res.error_for_status() {
        Ok(res) => res,
        Err(err) if err.status().is_some() => {
            if err.status().unwrap() == StatusCode::NOT_FOUND {
                return Ok(None);
            }

            return Err(err.into());
        }

        Err(e) => return Err(e.into()),
    };

    res.bytes().await.context("unable to get bytes from request").map(Some)
}

pub fn user<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    hash: Option<H>,
) -> impl Future<Output = eyre::Result<Option<Bytes>>> + '_ {
    from_entity(&ctx.storage, "users", "avatars", id, hash.map(Into::into))
}

pub fn organization<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    hash: Option<H>,
) -> impl Future<Output = eyre::Result<Option<Bytes>>> + '_ {
    from_entity(&ctx.storage, "organizations", "avatars", id, hash.map(Into::into))
}

pub fn repository<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    hash: Option<H>,
) -> impl Future<Output = eyre::Result<Option<Bytes>>> + '_ {
    from_entity(&ctx.storage, "repositories", "icons", id, hash.map(Into::into))
}

pub fn upload_user_avatar<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    data: Bytes,
) -> impl Future<Output = eyre::Result<String>> + '_ {
    upload(&ctx.storage, "users", "avatars", id, data)
}

pub fn upload_org_avatar<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    data: Bytes,
) -> impl Future<Output = eyre::Result<String>> + '_ {
    upload(&ctx.storage, "organizations", "avatars", id, data)
}

pub fn upload_repository_icon<H: Into<String>>(
    ctx: &ServerContext,
    id: u64,
    data: Bytes,
) -> impl Future<Output = eyre::Result<String>> + '_ {
    upload(&ctx.storage, "repositories", "icons", id, data)
}

#[instrument(name = "charted.ops.avatars.fetch", skip(storage))]
async fn from_entity(
    storage: &StorageService,
    entity: &str,
    class: &str,
    id: u64,
    hash: Option<String>,
) -> eyre::Result<Option<Bytes>> {
    init(storage)?;
    let hash = hash.unwrap_or(String::from("current.png"));

    storage
        .open(format!("./{entity}/{class}/{id}/{hash}"))
        .await
        .with_context(|| format!("failed to open file: ./{class}/{entity}/{id}/{hash}"))
}

#[instrument(name = "charted.ops.avatars.upload", skip(storage))]
async fn upload(storage: &StorageService, entity: &str, class: &str, id: u64, data: Bytes) -> eyre::Result<String> {
    let content_type = default_resolver(&data);
    let mut found = false;

    for ct in ACCEPTABLE_CONTENT_TYPE.iter() {
        if content_type.starts_with(ct) {
            found = true;
            break;
        }
    }

    if !found {
        return Err(eyre!("unacceptable content type received: {content_type}"));
    }

    let request = UploadRequest::default()
        .with_content_type(Some(content_type.clone()))
        .with_data(data)
        .with_metadata(azalia::hashmap! {
            "user.id" => id.to_string(),
            "service" => "Noelware/charted-server"
        });

    let hash = rand_string(4);
    let ext = match &*content_type.to_ascii_lowercase() {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        _ => unreachable!(),
    };

    storage
        .upload(format!("./{entity}/{class}/{id}/{hash}.{ext}"), request)
        .await
        .map(|_| hash)
        .context("failed to upload avatar")
}

fn compute_md5_hash(bytes: &[u8]) -> String {
    let mut hasher = md5::Md5::new();
    Digest::update(&mut hasher, bytes);

    faster_hex::hex_string(hasher.finalize_fixed().as_slice())
}
