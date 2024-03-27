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

use crate::{
    hashmap,
    server::models::res::{err, ApiResponse, ErrorCode},
};
use axum::http::StatusCode;
use serde_json::{json, Map, Value};
use std::borrow::Cow;
use validator::{ValidateEmail, ValidationError, ValidationErrors, ValidationErrorsKind};

/// Validates a function that has the signature of:
///
/// `fn(&R) -> Result<(), ValidationErrors>`
///
/// and returns a [`ApiResponse`] as the error variant.
pub fn validate<R, F: Fn(&R) -> Result<(), ValidationErrors>>(receiver: &R, func: F) -> Result<(), ApiResponse> {
    func(receiver).map_err(|e| {
        let mut paths = Map::new();
        for (key, err) in e.errors().iter() {
            paths.insert(key.to_string(), nest_err(err));
        }

        err(
            StatusCode::NOT_ACCEPTABLE,
            (ErrorCode::ValidationFailed, "failed to validate", Value::Object(paths)),
        )
    })
}

pub fn validate_email<'a, R: Into<Cow<'a, str>> + ValidateEmail>(receiver: R) -> Result<(), ApiResponse> {
    fn do_validate<'a, R: Into<Cow<'a, str>> + ValidateEmail>(email: &R) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        match email.validate_email() {
            true => Ok(()),
            false => {
                errors.add(
                    "email",
                    ValidationError {
                        code: Cow::Borrowed("INVALID_EMAIL"),
                        message: Some(Cow::Borrowed("received invalid email address")),
                        params: hashmap!(),
                    },
                );

                Err(errors)
            }
        }
    }

    validate(&receiver, do_validate)
}

fn nest_err(err: &ValidationErrorsKind) -> Value {
    match err {
        ValidationErrorsKind::Field(vec) => {
            let mut fields = vec![];
            for item in vec.iter() {
                let mut params = item.params.clone();
                let value = params.remove(&Cow::Borrowed("value"));
                let mut as_json = Map::new();
                for (key, value) in params.iter() {
                    as_json.insert(key.to_string(), value.clone());
                }

                fields.push(json!({
                    "message": match item.message.clone() {
                        Some(message) => Some(Value::String(message.to_string())),
                        None => match value {
                            Some(Value::String(s)) if !s.is_empty() => Some(Value::String(s)),
                            _ => None
                        }
                    },
                    "params": as_json
                }));
            }

            Value::Array(fields)
        }

        _ => unreachable!(),
    }
}
