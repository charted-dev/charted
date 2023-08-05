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

use crate::models::NameOrSnowflake as NameOrSnowflakeModel;
use async_trait::async_trait;
use axum::{
    extract::{rejection::RawPathParamsRejection, FromRequestParts, RawPathParams},
    http::{header, request::Parts, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

#[derive(Clone)]
pub enum NameOrSnowflakeError {
    PathParamsRejection(Arc<RawPathParamsRejection>),
    InvalidSnowflake(String, &'static str),
    InvalidName(String, &'static str),
    MissingParameter,
}

impl Debug for NameOrSnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathParamsRejection(params) => Debug::fmt(params, f),
            Self::InvalidSnowflake(id, why) => f.write_str(format!("invalid snowflake ({id}): {why}").as_str()),
            Self::InvalidName(name, why) => f.write_str(format!("invalid name ({name}): {why}").as_str()),
            Self::MissingParameter => f.write_str("unable to find `idOrName` path parameter"),
        }
    }
}

impl Display for NameOrSnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathParamsRejection(params) => Display::fmt(params, f),
            Self::InvalidSnowflake(id, why) => f.write_str(format!("invalid snowflake ({id}): {why}").as_str()),
            Self::InvalidName(name, why) => f.write_str(format!("invalid name ({name}): {why}").as_str()),
            Self::MissingParameter => f.write_str("unable to find `idOrName` path parameter"),
        }
    }
}

impl std::error::Error for NameOrSnowflakeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PathParamsRejection(params) => params.source(),
            _ => None,
        }
    }
}

impl IntoResponse for NameOrSnowflakeError {
    fn into_response(self) -> Response {
        let body = serde_json::to_string_pretty(&serde_json::json!({
            "success": false,
            "errors": [serde_json::json!({
                "code": "INVALID_ID_OR_NAME",
                "message": format!("{self}")
            })]
        }))
        .unwrap();

        let len = body.len().to_string();
        let headers: [(HeaderName, HeaderValue); 2] = [
            (header::CONTENT_LENGTH, HeaderValue::from_str(len.as_str()).unwrap()),
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            ),
        ];

        (StatusCode::BAD_REQUEST, headers, body).into_response()
    }
}

/// Axum extractor to extract `idOrName` path parameters and transform
/// them into [`NameOrSnowflake`][crate::models::NameOrSnowflake].
#[derive(Debug, Clone)]
pub struct NameOrSnowflake(NameOrSnowflakeModel);

#[async_trait]
impl<S> FromRequestParts<S> for NameOrSnowflake
where
    S: Send + Sync,
{
    type Rejection = NameOrSnowflakeError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params = RawPathParams::from_request_parts(parts, _state)
            .await
            .map_err(|e| NameOrSnowflakeError::PathParamsRejection(Arc::new(e)))?;

        match params.iter().find(|(name, _)| name == &"idOrName") {
            Some((_, value)) => {
                let mut is_snowflake = true;
                let nos = match value.parse::<u64>() {
                    Ok(id) => NameOrSnowflakeModel::Snowflake(id),
                    Err(_) => {
                        is_snowflake = false;
                        NameOrSnowflakeModel::Name(value.to_string())
                    }
                };

                match nos.is_valid() {
                    Ok(()) => Ok(NameOrSnowflake(nos)),
                    Err(why) => match is_snowflake {
                        false => Err(NameOrSnowflakeError::InvalidName(value.to_string(), why)),
                        true => Err(NameOrSnowflakeError::InvalidSnowflake(value.to_string(), why)),
                    },
                }
            }

            None => Err(NameOrSnowflakeError::MissingParameter),
        }
    }
}
