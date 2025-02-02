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

pub macro impl_list_response($name:ident as $ref:literal) {
    pub(crate) struct $name {
        _priv: (),
    }

    impl ::utoipa::PartialSchema for $name {
        fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::Schema> {
            let ::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Object(mut object)) =
                ::charted_core::api::Response::<()>::schema()
            else {
                ::core::unreachable!();
            };

            let _ = object.properties.remove("data");
            object.properties.insert(
                "data".into(),
                ::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Array(
                    ::utoipa::openapi::ArrayBuilder::new()
                        .items(::utoipa::openapi::RefOr::Ref(::utoipa::openapi::Ref::from_schema_name(
                            $ref,
                        )))
                        .build(),
                )),
            );

            ::utoipa::openapi::RefOr::T(::utoipa::openapi::Schema::Object(object))
        }
    }

    impl ::utoipa::ToSchema for $name {
        fn name() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed(::core::stringify!($name))
        }
    }
}

pub macro forward_schema_impl(for $ty:ty) {
    impl ::utoipa::PartialSchema for $ty {
        fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::Schema> {
            ::charted_core::api::Response::<$ty>::schema()
        }
    }

    impl ::utoipa::ToSchema for $ty {
        fn name() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed(stringify!($ty))
        }
    }
}
