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

use charted_core::api;
use utoipa::{
    Modify,
    openapi::{ComponentsBuilder, OpenApi},
};

/// utoipa modifier to include the default api revision's paths.
pub struct IncludeDefaultVersionWithoutPrefix;
impl Modify for IncludeDefaultVersionWithoutPrefix {
    fn modify(&self, openapi: &mut OpenApi) {
        let default_api_version = api::Version::default();
        let matcher = default_api_version.path();

        for (mut path, item) in openapi
            .paths
            .paths
            .clone()
            .into_iter()
            .filter(|(key, _)| key.starts_with(&matcher))
        {
            path = path.trim_start_matches(&matcher).to_owned();
            if path.is_empty() {
                path = "/".into();
            }

            openapi.paths.paths.insert(path, item);
        }
    }
}

pub struct IncludeErrorProneSchemas;
impl Modify for IncludeErrorProneSchemas {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut components =
            Into::<ComponentsBuilder>::into(unsafe { openapi.components.take().unwrap_unchecked() });

        error_prone_schemas! {components=>
            charted_types::Ulid,
            charted_types::DateTime,
            charted_core::serde::Duration,
            crate::openapi::Url
        }

        openapi.components = Some(components.build());
    }
}

macro_rules! error_prone_schemas {
    ($components:ident => $($Ty:ty),+) => {$(
        $components = ($components)
            .schema_from::<$Ty>()
            .schemas_from_iter({
                let mut schemas = ::std::vec::Vec::new();
                <$Ty as $crate::__macro_support::utoipa::ToSchema>::schemas(&mut schemas);

                schemas
            });
    )+};
}

pub(in crate::openapi::addons) use error_prone_schemas;
