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

use crate::{Env, mk_api_response_types, mk_into_responses, mk_list_based_api_response_types, mk_route_handler};
use axum::{extract::State, http::StatusCode};
use charted_core::api;
use charted_feature::Metadata;
use serde::Serialize;
use utoipa::ToSchema;

/// Datatype for an enabled feature.
#[derive(Clone, Serialize, ToSchema)]
pub struct EnabledFeature {
    /// If the feature is enabled.
    pub enabled: bool,

    /// Metadata about this feature.
    pub metadata: Metadata,
}

mk_api_response_types!(EnabledFeature);
mk_list_based_api_response_types!(EnabledFeature);
mk_into_responses!(for EnabledFeature {
    "200" => [ref(EnabledFeatureResponse)];
    "404" => [error(description = "404 Not Found")];
});

mk_route_handler! {
    /// Returns all the server's features.
    #[path("/v1/features", get, {
        operation_id = "getServerFeatures",
        responses(
            (
                status = 200,
                description = "list of all features and checks if they are enabled or not",
                body = ref("#/components/schemas/ListEnabledFeatureResponse")
            )
        )
    })]
    #[app_state = Env]
    fn features({State(_)}: State<Env>) -> api::Response<Vec<EnabledFeature>> {
        api::ok(StatusCode::OK, [].to_vec())
    }
}
