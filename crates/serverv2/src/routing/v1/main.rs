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

use crate::{mk_api_response_types, mk_into_responses};
use axum::http::StatusCode;
use charted_core::{BuildInfo, Distribution, api};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Main {
    /// current distribution
    #[schema(read_only)]
    distribution: Distribution,

    /// build information about the server
    #[schema(read_only)]
    build_info: BuildInfo,
}

impl Default for Main {
    fn default() -> Self {
        Main {
            distribution: Distribution::detect(),
            build_info: BuildInfo::new(),
        }
    }
}

mk_api_response_types!(Main);
mk_into_responses!(for Main {
    "200" => [ref(MainResponse)];
});

#[axum::debug_handler]
#[utoipa::path(get, path = "/v1", operation_id = "Main", responses(Main))]
pub async fn main() -> api::Response<Main> {
    api::from_default(StatusCode::OK)
}
