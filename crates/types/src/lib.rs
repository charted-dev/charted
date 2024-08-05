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

//! The `charted-types` crate defines types that can be used within the lifecycle
//! of the API server.

pub mod db;
pub mod helm;
pub mod name;
pub mod payloads;

mod distribution;
pub use distribution::*;

use utoipa::openapi::{KnownFormat, ObjectBuilder, RefOr, Schema, SchemaFormat, SchemaType};

/// Represents a generic [`chrono::DateTime`] that uses the local time.
pub type DateTime = chrono::DateTime<chrono::Local>;

/// OpenAPI schema for [`chrono::DateTime`].
pub fn datetime<'s>() -> (&'s str, RefOr<Schema>) {
    let schema = Schema::Object(
        ObjectBuilder::new()
            .schema_type(SchemaType::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
            .description(Some("ISO 8601 combined date and time using local time"))
            .build(),
    );

    ("DateTime", RefOr::T(schema))
}
