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

use crate::openapi::ApiResponse;
use charted_types::{ApiKey, Organization, OrganizationMember, Repository, RepositoryMember, Ulid};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::{
    IntoParams, ToResponse, ToSchema,
    openapi::{Array, KnownFormat, Object, Ref, RefOr, Response, Schema, SchemaFormat, Type, schema::SchemaType},
};

pub type PaginatedRepository = Paginated<Repository>;
pub type PaginatedRepositoryMember = Paginated<RepositoryMember>;
pub type PaginatedOrganization = Paginated<Organization>;
pub type PaginatedOrganizationMember = Paginated<OrganizationMember>;
pub type PaginatedApiKey = Paginated<ApiKey>;

/// The ordering of the data. Either ascending or descending.
#[derive(Debug, Clone, Copy, Default, Deserialize, ToSchema)]
pub enum Ordering {
    #[serde(rename = "ASC")]
    #[default]
    Ascending,

    #[serde(rename = "DESC")]
    Descending,
}

/// A pagination request.
#[derive(Debug, Clone, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PaginationRequest {
    /// Amount of entries per page.
    #[serde(default = "__per_page")]
    #[schema(minimum = 10, maximum = 100)]
    pub per_page: usize,

    /// Ordering of the data.
    #[serde(default)]
    pub order_by: Ordering,

    /// Before cursor.
    #[serde(default)]
    pub cursor: Option<Ulid>,
}

/// A paginated response.
#[derive(Debug, Clone, Serialize)]
pub struct Paginated<T> {
    /// The list of entries, if any.
    pub entries: Vec<T>,

    /// A cursor that returns the next position in the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Ulid>,

    /// A cursor that returns to the previous position in the page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_cursor: Option<Ulid>,
}

impl<T: ToSchema> utoipa::__dev::ComposeSchema for Paginated<T> {
    fn compose(_: Vec<RefOr<Schema>>) -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            Object::builder()
                .description(Some(format!("Paginated response for schema {}", T::name())))
                .property(
                    "entries",
                    RefOr::T(Schema::Array(Array::new(RefOr::Ref(Ref::from_schema_name(T::name()))))),
                )
                .required("entries")
                .property(
                    "next_cursor",
                    Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::Array(vec![Type::String, Type::Null]))
                            .description(Some("A cursor that returns the next position in the page."))
                            .read_only(Some(true))
                            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Ulid)))
                            .build(),
                    ),
                )
                .property(
                    "previous_cursor",
                    Schema::Object(
                        Object::builder()
                            .schema_type(SchemaType::Array(vec![Type::String, Type::Null]))
                            .description(Some("A cursor that returns to the previous position in the page."))
                            .read_only(Some(true))
                            .format(Some(SchemaFormat::KnownFormat(KnownFormat::Ulid)))
                            .build(),
                    ),
                )
                .build(),
        ))
    }
}

impl<T: ToSchema> ToSchema for Paginated<T> {
    fn name() -> Cow<'static, str> {
        Cow::Owned(format!("Paginated{}", T::name()))
    }
}

impl<'r, T: ToSchema> ToResponse<'r> for Paginated<T> {
    fn response() -> (Cow<'r, str>, RefOr<Response>) {
        let name = Cow::Owned(format!("Paginated{}", T::name()));
        let response = ApiResponse::<Paginated<T>>::response().1;

        (name, response)
    }
}

const fn __per_page() -> usize {
    10
}
