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

use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

/// The ordering to use when querying paginated REST calls.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
pub enum OrderBy {
    /// Sorts all entities in ascending order by the entity ID
    #[serde(rename = "ASC")]
    #[default]
    Ascending,

    /// Sorts all entities in descending order by the entity ID
    #[serde(rename = "DESC")]
    Descending,
}

impl Display for OrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderBy::Ascending => f.write_str("ASC"),
            OrderBy::Descending => f.write_str("DESC"),
        }
    }
}

/// Represents the result of a paginated REST call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination<T> {
    /// List of all the data that was contained by the cursors.
    pub data: Vec<T>,

    /// Page information relating to this pagination request.
    pub page_info: PageInfo,
}

impl<T> Default for Pagination<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            page_info: PageInfo { cursor: None },
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PaginationQuery {
    /// How many elements should be present in a page.
    #[serde(default = "default_per_page")]
    pub per_page: usize,

    /// Ordering to use when paginating REST calls.
    #[serde(default)]
    pub order: OrderBy,

    /// Cursor to passthrough to proceed into the next or previous page. If this is
    /// not provided, this will be the "first" page.
    #[serde(default)]
    pub cursor: Option<u64>,
}

fn default_per_page() -> usize {
    10
}

/// Information about a [pagination][Pagination] page.
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PageInfo {
    /// The next cursor, which will always be a [Snowflake]. If this is `null`, then
    /// there is no more pages to paginate through.
    #[schema(value_type = Snowflake)]
    pub cursor: Option<u64>,
}

macro_rules! gen_response_schemas_for_types {
    ($ty:ty) => {
        paste::paste! {
            pub struct [<Paginated $ty>];
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

            pub struct [<$ty PaginatedResponse>];
            charted_proc_macros::generate_response_schema!([<$ty PaginatedResponse>], schema = stringify!([<Paginated $ty>]));
        }
    };
}

gen_response_schemas_for_types!(RepositoryRelease);
gen_response_schemas_for_types!(Organization);
gen_response_schemas_for_types!(Repository);
gen_response_schemas_for_types!(Member);
gen_response_schemas_for_types!(ApiKey);
