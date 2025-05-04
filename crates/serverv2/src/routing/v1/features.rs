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

use crate::{feature::Metadata, mk_api_response_types, mk_into_responses};
use serde::Serialize;
use utoipa::ToSchema;

/// Datatype for an enabled feature.
#[derive(Serialize, ToSchema)]
pub struct EnabledFeature {
    /// If the feature is enabled.
    pub enabled: bool,

    /// Metadata about this feature.
    pub metadata: Metadata,
}

mk_api_response_types!(EnabledFeature);
mk_into_responses!(for EnabledFeature {
    "200" => [ref(EnabledFeatureResponse)];
    "404" => [error(description = "404 Not Found")];
});
