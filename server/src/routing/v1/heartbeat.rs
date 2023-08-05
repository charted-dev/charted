// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use serde_json::Value;
use utoipa::openapi::{
    path::{OperationBuilder, PathItemBuilder},
    ContentBuilder, ObjectBuilder, PathItem, PathItemType, RefOr, ResponseBuilder, Schema, SchemaType,
};

pub async fn heartbeat() -> &'static str {
    "Ok."
}

pub fn paths() -> PathItem {
    PathItemBuilder::new()
        .operation(
            PathItemType::Get,
            OperationBuilder::new()
                .operation_id(Some("heartbeat"))
                .description(Some("Health endpoint to use to check if the server is available to receive requests. Useful for Docker's healthcheck or Kubernetes' readiness/liveness/startup probes."))
                .response(
                    "200",
                    RefOr::T(
                        ResponseBuilder::new()
                            .description("Successful response.")
                            .content(
                                "application/json",
                                ContentBuilder::new()
                                    .schema(RefOr::T(
                                        Schema::Object(
                                            ObjectBuilder::new()
                                                .schema_type(SchemaType::String)
                                                .example(Some(Value::String("Ok.".into())))
                                                .build()
                                            )
                                        )
                                    ).build()
                            )
                            .build()
                    )
                )
                .build()
        )
        .build()
}
