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

use axum::{
    extract::{rejection::QueryRejection, FromRequestParts},
    http::request::Parts,
};
use charted_core::api;
use serde::de::DeserializeOwned;

/// `Query` is a newtype wrapper for [`axum::extract::Query`], in which it uses
/// [`api::Response`] to transmit errors properly.
pub struct Query<T>(pub T);
impl<S: Send + Sync, T: DeserializeOwned> FromRequestParts<S> for Query<T> {
    type Rejection = api::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        ::axum::extract::Query::<T>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(x)| Self(x))
            .map_err(|e| {
                tracing::error!(error = %e, "failed to parse query parameter");
                sentry::capture_error(&e);

                match e {
                    QueryRejection::FailedToDeserializeQueryString(err) => api::err(
                        err.status(),
                        (api::ErrorCode::ParsingQueryParamsFailed, err.body_text()),
                    ),

                    _ => unreachable!(),
                }
            })
    }
}
