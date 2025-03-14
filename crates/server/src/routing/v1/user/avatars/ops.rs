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

use crate::{Context, HTTP_CLIENT};
use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode, header},
};
use azalia::remi::{
    core::{Blob, File, StorageService},
    fs,
};
use charted_core::api;
use charted_types::{Ulid, User};
use serde_json::json;

const DICEBEAR_IDENTICONS_URI: &str = "https://avatars.dicebear.com/api/identicon";
const GRAVATAR_URI: &str = "https://secure.gravatar.com/avatar";

type OpaqueRT = ([(header::HeaderName, header::HeaderValue); 1], Bytes);

charted_core::assert_into_response!(OpaqueRT);

pub(crate) async fn fetch(cx: Context, user: User) -> Result<OpaqueRT, api::Response> {
    if let Some(email) = user.gravatar_email {
        if let Some(ref hash) = user.avatar_hash &&
            !user.prefers_gravatar
        {
            return fetch_by_hash(cx, user.id, hash.to_owned()).await;
        }

        return fetch_gravatar(&email).await;
    } else if let Some(hash) = user.avatar_hash {
        return fetch_by_hash(cx, user.id, hash).await;
    }

    let url = format!("{DICEBEAR_IDENTICONS_URI}/{}.png", user.id);
    debug!(%url, "requesting avatar from gravatar");

    let res = HTTP_CLIENT.get(url).send().await.map_err(api::system_failure)?;
    if res.status() == StatusCode::NOT_FOUND {
        return Err(api::Response {
            success: false,
            data: None,
            errors: Vec::new(),
            headers: HeaderMap::new(),
            status: StatusCode::NOT_FOUND,
        });
    }

    let ct = res.headers().get(header::CONTENT_TYPE).unwrap();

    Ok((
        [(header::CONTENT_TYPE, ct.clone())],
        res.bytes().await.map_err(api::system_failure)?,
    ))
}

pub(crate) async fn fetch_gravatar(email: &str) -> Result<OpaqueRT, api::Response> {
    let hash = {
        use md5::compute;

        let computed = compute(email.as_bytes());
        hex::encode(*computed)
    };

    let url = format!("{GRAVATAR_URI}/{hash}.png");
    debug!(%url, "requesting avatar from gravatar");

    let res = HTTP_CLIENT.get(url).send().await.map_err(api::system_failure)?;
    if res.status() == StatusCode::NOT_FOUND {
        return Err(api::Response {
            success: false,
            data: None,
            errors: Vec::new(),
            headers: HeaderMap::new(),
            status: StatusCode::NOT_FOUND,
        });
    }

    let ct = res.headers().get(header::CONTENT_TYPE).unwrap();

    Ok((
        [(header::CONTENT_TYPE, ct.clone())],
        res.bytes().await.map_err(api::system_failure)?,
    ))
}

#[instrument(name = "charted.server.avatars.getByHash", skip_all, fields(repr = "User", %id))]
pub(crate) async fn fetch_by_hash(cx: Context, id: Ulid, hash: String) -> Result<OpaqueRT, api::Response> {
    match cx.storage.blob(format!("./avatars/users/{id}/{hash}")).await {
        Ok(Some(Blob::File(File { data, .. }))) => {
            let ct = fs::default_resolver(&data);
            let mime = ct
                .parse::<mime::Mime>()
                .inspect_err(|e| {
                    error!(error = %e, "failed to validate content type of data from storage server");
                    sentry::capture_error(e);
                })
                .map_err(api::system_failure)?;

            if mime.type_() != mime::IMAGE {
                return Err(api::err(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    (
                        api::ErrorCode::InvalidContentType,
                        "media type for image was not an image",
                        json!({"mediaType": mime.to_string()}),
                    ),
                ));
            }

            Ok((
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime.type_().as_str()).unwrap(),
                )],
                data,
            ))
        }

        Ok(Some(_)) => unreachable!(),
        Ok(None) => Err(api::err(
            StatusCode::NOT_FOUND,
            (
                api::ErrorCode::EntityNotFound,
                "avatar hash was not found",
                json!({"hash": hash}),
            ),
        )),

        Err(e) => {
            error!(error = %e, "storage server returned an error");
            sentry::capture_error(&e);

            Err(api::system_failure(e))
        }
    }
}
