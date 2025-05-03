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
                        $code => {
                            let __ret = $crate::__internal_mk_into_responses!(@internal $($tt)*);
                            __ret
                        }
                    )+
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_mk_into_responses {
    (@internal ref($T:ty)) => {
        $crate::__macro_support::utoipa::openapi::RefOr::Ref(
            $crate::__macro_support::utoipa::openapi::Ref::from_response_name(
                <$T as $crate::__macro_support::utoipa::ToResponse<'_>>::response().0,
            ),
        )
    };
}
