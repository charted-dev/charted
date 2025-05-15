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
            if !(field.len() <= $len) {
                ::std::vec::Vec::push(&mut $errors, ::charted_core::api::Error {
                    code: ::charted_core::api::ErrorCode::ValidationFailed,
                    message: ::std::borrow::Cow::Borrowed(concat!("expected to be less than ", $len, " characters")),
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

#[macro_export]
macro_rules! mk_into_responses {
    (for $Ty:ty {$(
        $code:expr => [$($tt:tt)*];
    )+}) => {
        impl $crate::__macro_support::utoipa::IntoResponses for $Ty {
            fn responses() -> ::std::collections::BTreeMap<
                String,
                $crate::__macro_support::utoipa::openapi::RefOr<
                    $crate::__macro_support::utoipa::openapi::Response
                >
            > {
                ::azalia::btreemap! {
                    $(
                        $code => $crate::__internal_mk_into_responses!(@internal $($tt)*)
                    ),+
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_mk_into_responses {
    (@internal ref(with $content:literal => $T:ty$(; $($field:ident($value:expr);)*)?)) => {
        $crate::__macro_support::utoipa::openapi::Response::builder()
            .content(
                $content,
                $crate::__macro_support::utoipa::openapi::Content::builder()
                    .schema(::core::option::Option::Some(
                        $crate::__macro_support::utoipa::openapi::RefOr::Ref(
                            $crate::__macro_support::utoipa::openapi::Ref::from_schema_name(
                                <$T as $crate::__macro_support::utoipa::ToSchema>::name()
                            )
                        )
                    ))
                    .build()
            )
            $(
                $(
                    .$field($value)
                )*
            )?
            .build()
    };

    (@internal ref($T:ty)) => {
        $crate::__macro_support::utoipa::openapi::RefOr::Ref(
            $crate::__macro_support::utoipa::openapi::Ref::from_response_name(
                <$T as $crate::__macro_support::utoipa::ToResponse<'_>>::response().0,
            ),
        )
    };

    (@internal error) => {
        $crate::__internal_mk_into_responses(@internal ref($crate::openapi::ApiErrorResponse))
    };

    (@internal error($($field:ident($value:expr)),*)) => {{
        #[allow(unused_mut)]
        let mut response = $crate::extract_refor_t!(
            <$crate::openapi::ApiErrorResponse as $crate::__macro_support::utoipa::ToResponse>::response().1
        );

        $(
            $crate::modify_property!(
                response.$field($value)
            );
        )*

        response
    }};
}
