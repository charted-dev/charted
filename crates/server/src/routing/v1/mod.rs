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

pub mod heartbeat;
pub mod index;
pub mod info;
pub mod main;
pub mod openapi;
pub mod user;

use crate::ServerContext;
use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing, Router};
use charted_core::{api, VERSION};
use serde::Serialize;
use serde_json::json;
use std::{borrow::Cow, ops::Deref};
use utoipa::ToSchema;

/// Generic entrypoint message for any API route like `/users`.
#[derive(Serialize, ToSchema)]
pub struct Entrypoint {
    /// Humane message to greet you.
    pub message: Cow<'static, str>,

    /// URI to the documentation for this entrypoint.
    pub docs: Cow<'static, str>,
}

impl Entrypoint {
    pub fn new(entity: impl AsRef<str>) -> Self {
        let entity = entity.as_ref();
        Self {
            message: Cow::Owned(format!("welcome to the {entity} API")),
            docs: Cow::Owned(format!(
                "https://charts.noelware.org/docs/server/{VERSION}/api/reference/{}",
                entity.to_lowercase().replace(' ', "")
            )),
        }
    }
}

pub fn create_router(cx: &ServerContext) -> Router<ServerContext> {
    let mut router = Router::new()
        .nest("/users", user::create_router())
        .route("/indexes/{idOrName}", routing::get(index::get_chart_index))
        .route("/heartbeat", routing::get(heartbeat::heartbeat))
        .route("/openapi.json", routing::get(openapi::openapi))
        .route("/info", routing::get(info::info))
        .route("/", routing::get(main::main))
        .fallback(fallback);

    for feature in &cx.features {
        router = feature.extend_router().with_state(cx.deref().clone());
    }

    router
}

async fn fallback(req: Request) -> impl IntoResponse {
    api::err(
        StatusCode::NOT_FOUND,
        (
            api::ErrorCode::HandlerNotFound,
            "endpoint was not found",
            json!({
                "method": req.method().as_str(),
                "uri": req.uri().path()
            }),
        ),
    )
}
