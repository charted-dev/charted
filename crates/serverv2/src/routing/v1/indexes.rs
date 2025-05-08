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

use crate::{
    Env, OwnerExt, ext::ResultExt, extract::Path, mk_api_response_types, mk_into_responses, mk_route_handler,
};
use axum::{extract::State, http::StatusCode};
use charted_core::api::{self, Yaml};
use charted_helm_charts::DataStoreExt;
use charted_helm_types::ChartIndex;
use charted_types::{NameOrUlid, Owner};

mk_api_response_types!(ChartIndex);
mk_into_responses!(for ChartIndexResponse {
    "200" => [ref(
        with "text/yaml" => ChartIndex;
            description("200 OK");
    )];

    "404" => [error(description("User or Organization was not found"))];
    "5XX" => [error(description("Internal Server Error"))];
});

mk_route_handler! {
    /// Retrieve a chart index from a **User** or **Organization**.
    #[path("/v1/indexes/{idOrName}", get, {
        operation_id = "getChartIndex",
        tag = "Main",
        params(NameOrUlid),
        responses(ChartIndexResponse)
    })]
    #[app_state = Env]
    fn fetch({State(env)}: State<Env>, {Path(id_or_name)}: Path<NameOrUlid>) -> Result<Yaml<ChartIndex>, api::Response> {
        let Some(owner) = Owner::query_by_id_or_name(&env, id_or_name).await.into_system_failure()? else {
            return Err(api::err(
                StatusCode::NOT_FOUND,
                (
                    api::ErrorCode::EntityNotFound,
                    "user or organization by id or name was not found"
                )
            ));
        };

        let metadata = env.ds.metadata();
        let index = metadata
            .get_chart_index(owner.id())
            .await
            .map_err(api::system_failure_from_report)?
            .unwrap_or_default();

        Ok(Yaml::new(StatusCode::OK, index))
    }
}
