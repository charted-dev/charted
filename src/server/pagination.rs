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

macro_rules! gen_response_schemas_for_types {
    ($ty:ty) => {
        paste::paste! {
            pub(crate) struct [<Paginated $ty>];
            impl<'s> ::utoipa::ToSchema<'s> for [<Paginated $ty>] {
                fn schema() -> (&'s str, ::utoipa::openapi::RefOr<::utoipa::openapi::Schema>) {
                    (
                        concat!("Paginated", stringify!($ty)),
                        ::utoipa::openapi::RefOr::T(
                            ::utoipa::openapi::Schema::Object(
                                ::utoipa::openapi::ObjectBuilder::new()
                                    .property(
                                        "data",
                                        ::utoipa::openapi::ArrayBuilder::new()
                                            .items(::utoipa::openapi::Ref::from_schema_name(stringify!($ty)))
                                            .build()
                                    )
                                    .required("data")
                                    .property(
                                        "page_info",
                                        ::utoipa::openapi::Ref::from_schema_name("PageInfo")
                                    )
                                    .required("page_info")
                                    .build()
                            )
                        )
                    )
                }
            }

            pub(crate) struct [<$ty PaginatedResponse>];
            charted_proc_macros::generate_response_schema!([<$ty PaginatedResponse>], schema = stringify!([<Paginated $ty>]));
        }
    };
}

gen_response_schemas_for_types!(Organization);
gen_response_schemas_for_types!(Repository);
gen_response_schemas_for_types!(Member);
