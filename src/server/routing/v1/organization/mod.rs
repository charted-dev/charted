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

pub mod icons;
pub mod repositories;

use super::EntrypointResponse;
use crate::{
    common::models::{
        entities::{ApiKeyScope, ApiKeyScopes, Organization},
        NameOrSnowflake,
    },
    db::controllers::DbController,
    openapi::generate_response_schema,
    server::{
        controller,
        middleware::session::{Middleware, Session},
        models::res::{err, internal_server_error, ok, ErrorCode, Result},
        validation::validate,
    },
    Instance,
};
use axum::{extract::State, handler::Handler, http::StatusCode, routing, Extension, Router};
use serde_json::json;
use tower_http::auth::AsyncRequireAuthorizationLayer;
use validator::Validate;

pub struct OrganizationResponse;
generate_response_schema!(OrganizationResponse, schema = "Organization");

pub fn create_router() -> Router<Instance> {
    Router::new()
        .route("/", routing::get(EntrypointRestController::run))
        .route("/:idOrName", routing::get(GetOrgByIdOrNameRestController::run))
        .route(
            "/:idOrName/repositories",
            routing::get(repositories::ListOrgRepositoriesRestController::run.layer(
                AsyncRequireAuthorizationLayer::new(Middleware {
                    allow_unauthenticated_requests: true,
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoCreate]),
                    ..Default::default()
                }),
            ))
            .put(repositories::CreateOrganizationRepositoryRestController::run.layer(
                AsyncRequireAuthorizationLayer::new(Middleware {
                    scopes: ApiKeyScopes::with_iter([ApiKeyScope::RepoCreate]),
                    ..Default::default()
                }),
            )),
        )
        .route(
            "/:idOrName/icon",
            routing::get(icons::GetCurrentOrgIconRestController::run),
        )
        .route(
            "/:idOrName/icon/:hash",
            routing::get(icons::GetOrgIconByHashRestController::run),
        )
}

/// Entrypoint for the Organizations API
#[controller(id = "repositories", tags("Organizations"), response(200, "Successful response", ("application/json", response!("EntrypointResponse"))))]
pub async fn entrypoint() {
    ok(StatusCode::OK, EntrypointResponse::new("Organizations"))
}

/// Finds an organization by its ID or name.
#[controller(
    tags("Organizations"),
    securityRequirements(
        ("ApiKey", ["org:access"]),
        ("Bearer", []),
        ("Basic", [])
    ),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a `Name` or Snowflake ID"),
    response(200, "Successful response", ("application/json", response!("OrganizationResponse"))),
    response(403, "You are not allowed to see this resource", ("application/json", response!("ApiErrorResponse"))),
    response(404, "Entity was not found", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn get_org_by_id_or_name(
    State(Instance { controllers, .. }): State<Instance>,
    crate::server::extract::NameOrSnowflake(nos): crate::server::extract::NameOrSnowflake,
    session: Option<Extension<Session>>,
) -> Result<Organization> {
    validate(&nos, NameOrSnowflake::validate)?;
    let org = match controllers.organizations.get_by(&nos).await {
        Ok(Some(org)) => org,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "unable to find organization by id",
                    json!({"idOrName":nos}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    if org.private {
        if let Some(ref session) = session {
            if org.owner != session.user.id {
                return Err(err(
                    StatusCode::FORBIDDEN,
                    (
                        ErrorCode::AccessNotPermitted,
                        "you're not allowed to see this resource",
                        json!({"class":"Organization"}),
                    ),
                ));
            }
        }
    }

    Ok(ok(StatusCode::OK, org))
}
