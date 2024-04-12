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

use crate::{err, extract::Path, ApiResponse, ErrorCode};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use charted_entities::NameError;
use serde_json::json;

/// Extractor to [`Path`][axum::extract::Path] to extract it as a [`NameOrSnowflake`].
#[derive(Debug, Clone)]
pub struct NameOrSnowflake(pub charted_entities::NameOrSnowflake);

#[async_trait]
impl<S> FromRequestParts<S> for NameOrSnowflake
where
    S: Send + Sync,
{
    type Rejection = ApiResponse;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<charted_entities::NameOrSnowflake>::from_request_parts(parts, state).await {
            Ok(Path(nos)) => match nos {
                charted_entities::NameOrSnowflake::Name(name) => {
                    // Ok, this defeats the purpose of having a `NameOrSnowflake` variant
                    // if `nos` will always be a `Name`, but this is what happens when `Path<NameOrSnowflake>`
                    // is used; which hence why this extractor exists.
                    //
                    // Since `Path`'s `deserialize_any` goes to `deserialize_str`, we will need
                    // to validate if `name` is a u64, which will be a Snowflake.
                    if let Ok(id) = name.parse::<u64>() {
                        return Ok(Self(charted_entities::NameOrSnowflake::Snowflake(id)));
                    }

                    match name.is_valid() {
                        Ok(()) => Ok(Self(charted_entities::NameOrSnowflake::Name(name))),
                        Err(e) => match e {
                            NameError::InvalidCharacter { input, at, ch } => Err(err(
                                StatusCode::BAD_REQUEST,
                                (
                                    ErrorCode::InvalidType,
                                    "unexpected character when trying to parse `input`",
                                    json!({
                                        "input": input,
                                        "at": at,
                                        "char": ch
                                    }),
                                ),
                            )),

                            NameError::ExceededMax(over) => Err(err(
                                StatusCode::BAD_REQUEST,
                                (
                                    ErrorCode::BadRequest,
                                    "exceeded `name` length",
                                    json!({
                                        "went_over": over
                                    }),
                                ),
                            )),

                            NameError::InvalidUtf8 => Err(err(
                                StatusCode::BAD_REQUEST,
                                (ErrorCode::InvalidUtf8, "wanted valid utf-8 when parsing names"),
                            )),

                            NameError::Empty => Err(err(
                                StatusCode::BAD_REQUEST,
                                (ErrorCode::BadRequest, "name given was empty!"),
                            )),
                        },
                    }
                }

                ref_ => Ok(Self(ref_)),
            },

            Err(e) => Err(e),
        }
    }
}

impl From<NameOrSnowflake> for charted_entities::NameOrSnowflake {
    fn from(value: NameOrSnowflake) -> Self {
        value.0
    }
}
