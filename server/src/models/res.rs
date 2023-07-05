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
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where
    T: Serialize + Debug,
{
    #[serde(skip)]
    status: StatusCode,

    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}

impl<T: Serialize + Debug> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let mut res = Response::new(serde_json::to_string(&self).unwrap());
        *res.status_mut() = self.status;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    message: String,
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Empty {}

impl Error {
    pub(crate) fn new(code: &str, message: &str) -> Error {
        Error {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl From<(&str, &str)> for Error {
    fn from((code, message): (&str, &str)) -> Self {
        Error::new(code, message)
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
