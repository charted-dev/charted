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

/// Extracts the `T` from [`RefOr`].
#[macro_export]
macro_rules! extract_refor_t {
    ($val:expr) => {
        match $val {
            val => {
                let ::utoipa::openapi::RefOr::T(v) = val else {
                    unreachable!()
                };

                v
            }
        }
    };
}

#[macro_export]
macro_rules! modify_property {
    ($($val:ident.$field:ident($arg:expr))*) => {
        $($val.$field = From::from($arg);)*
    };
}

#[macro_export]
macro_rules! commit_patch {
    ($model:ident of bool: old.$oldf:ident => $newf:expr; [$entity:ident]) => {
        if let ::core::option::Option::Some(field) = $newf
            && ($entity).$oldf != field {
            $model.$oldf = ::sea_orm::ActiveValue::set(field);
        }
    };

    ($model:ident of string?: old.$oldf:ident => $newf:expr) => {
        if let ::core::option::Option::Some(field) = ($newf).as_deref() {
            let old = ($oldf).as_deref();
            if old.is_none() && !field.is_empty() {
                $model.$oldf = ::sea_orm::ActiveValue::set(Some(field.to_owned()));
            } else if let Some(old) = old &&
                !old.is_empty() &&
                old != field
            {
                $model.$oldf = ::sea_orm::ActiveValue::set(Some(field.to_owned()));
            } else {
                $model.$oldf = ::sea_orm::ActiveValue::set(None);
            }
        }
    };

    ($model:ident of string?: old.$oldf:ident => $newf:expr; validate that len < $len:literal [$errors:ident]) => {
        if let ::core::option::Option::Some(field) = ($newf).as_deref() {
            if field.len() < $len {
                ::std::vec::Vec::push(&mut $errors, ::charted_core::api::Error {
                    code: ::charted_core::api::ErrorCode::ValidationFailed,
                    message: ::std::borrow::Cow::Borrowed(concat!("expected to be less than ", $len, "characters")),
                    details: ::core::option::Option::Some(::serde_json::json!({
                        "path": stringify!($oldf),
                        "expected": $len,
                        "received": [field.len() - $len, $len]
                    }))
                });
            }
        } else {
            $crate::commit_patch!($model of string?: old.$oldf => $newf);
        }
    };

    ($model:ident of string?: old.$oldf:ident => $newf:expr; validate that $validator:expr) => {
        if let ::core::option::Option::Some(field) = ($newf).as_deref() {
            if !($validator) {
                ::std::vec::Vec::push(&mut $errors, ::charted_core::api::Error {
                    code: ::charted_core::api::ErrorCode::ValidationFailed,
                    message: ::std::borrow::Cow::Borrowed("validation failed"),
                    details: ::core::option::Option::Some(::serde_json::json!({
                        "path": stringify!($oldf),
                        "value": field.clone()
                    }))
                });
            }
        } else {
            $crate::commit_patch!($model of string?: old.$oldf => $newf);
        }
    };
}

/*
            let len = description.len();
            errors.push(api::Error {
                code: api::ErrorCode::ValidationFailed,
                message: Cow::Borrowed("expected to be less than 140 characters"),
                details: Some(json!({
                    "path": "description",
                    "expected": 140,
                    "received": [len - 140, len]
                })),
            });
*/
