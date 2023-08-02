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

use std::fmt::Debug;

use axum::{
    headers::HeaderValue,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ApiResponse<T>
where
    T: Serialize + Debug,
{
    #[serde(skip)]
    status: StatusCode,

    /// Indicates whether if this response was a success or not.
    pub success: bool,

    /// Optional data that was attached to this payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// List of errors that might've occurred when this request
    /// was being processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}

impl<T: Serialize + Debug> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let mut res = Response::new(serde_json::to_string(&self).unwrap());
        *res.status_mut() = self.status;
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res.into_response()
    }
}

// this is needed so that server/mod.rs:66 can be compiled (without it, it fails)
#[allow(clippy::from_over_into)]
impl<T: Serialize + Debug> Into<Response> for ApiResponse<T> {
    fn into(self) -> Response {
        self.into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Error {
    code: String,
    message: String,
    details: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {}

impl Error {
    pub(crate) fn new(code: &str, message: &str) -> Error {
        Error {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub(crate) fn new_with_details(code: &str, message: &str, details: Value) -> Error {
        Error {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }
}

impl From<(&str, &str)> for Error {
    fn from((code, message): (&str, &str)) -> Self {
        Error::new(code, message)
    }
}

impl From<(&str, &str, Value)> for Error {
    fn from((code, message, details): (&str, &str, Value)) -> Self {
        Error::new_with_details(code, message, details)
    }
}

pub fn ok<T: Serialize + Debug>(status: StatusCode, data: T) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        errors: None,
        status,
        data: Some(data),
    }
}

pub fn err(status: StatusCode, error: Error) -> ApiResponse<Empty> {
    ApiResponse {
        success: false,
        errors: Some(vec![error]),
        status,
        data: None,
    }
}
