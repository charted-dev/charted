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

use crate::{err, internal_server_error, ApiResponse, ErrorCode};
use async_trait::async_trait;
use axum::{
    extract::{rejection::QueryRejection, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use serde::de::DeserializeOwned;
use std::ops::Deref;
use tracing::error;

/// Newtype of the [`axum::extract::Query`] extractor but reports all rejections as [`ApiResponse`]s, which is more
/// useful than a simple message.
#[derive(Clone)]
pub struct Query<T>(pub T);
impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(value)) => Ok(Self(value)),
            Err(e) => {
                error!(error = %e, "failed to parse query parameter");
                sentry::capture_error(&e);

                match e {
                    QueryRejection::FailedToDeserializeQueryString(_) => Err(err(
                        StatusCode::BAD_REQUEST,
                        (ErrorCode::ParsingQueryParamsFailed, "failed to parse query parameters"),
                    )),

                    _ => Err(internal_server_error()),
                }
            }
        }
    }
}
