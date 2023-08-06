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

use std::ops::Deref;

use crate::models::res::{err, ApiResponse};
use async_trait::async_trait;
use axum::{extract::FromRequest, http::Request, BoxError};
use serde::de::DeserializeOwned;

/// Wrapper for [`axum::Json`] that uses charted-server's [API response][ApiResponse]
/// for the details on why it failed to parse JSON from the request
/// body.
pub struct Json<T>(pub T);

impl<T> Deref for Json<T>
where
    T: DeserializeOwned,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Json<T>
where
    T: serde::de::DeserializeOwned,
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(payload) => Ok(Json(payload.0)),
            Err(e) => Err(err(e.status(), ("UNABLE_TO_PARSE_BODY", e.to_string().as_str()).into())),
        }
    }
}
