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

/// Lists all the avaliable user repositories.
///
/// If the user is logged in with credentials, this will also show their private
/// repositories as well.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/repositories",
    operation_id = "listRepositories",
    tag = "Repositories"
)]
pub async fn list_user_repositories() {}

/// Lists all of this user's repositories.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,
    path = "/v1/users/@me/repositories",
    operation_id = "listMyRepositories",
    tag = "Repositories"
)]
pub async fn list_self_user_repositories() {}

/// Creates a repository under this user.
#[utoipa::path(
    put,
    path = "/v1/users/@me/repositories",
    operation_id = "createRepository",
    tag = "Repositories"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create_user_repository() {}

/*
/// Lists all the avaliable user repositories.
///
/// If the user is logged in with credentials, this will also show their private repositories as well.
#[utoipa::path(
    get,
    path = "/v1/users/{idOrName}/repositories",
    operation_id = "listRepositories",
    tag = "Repositories"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn list_user_repositories() {}

/// Lists all of this user's repositories.
#[utoipa::path(
    get,
    path = "/v1/users/@me/repositories",
    operation_id = "listMyRepositories",
    tag = "Repositories"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn list_self_user_repositories() {}

/// Creates a repository under this user.
#[utoipa::path(
    put,
    path = "/v1/users/@me/repositories",
    operation_id = "createRepository",
    tag = "Repositories"
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn create_user_repository() {}

/*
/// Retrieve all of a user's repositories. This filters out private ones.
#[controller(
    tags("Users", "Repositories"),
    response(200, "List of all the user's repositories", ("application/json", response!("RepositoryPaginatedResponse"))),
    pathParameter("idOrName", schema!("NameOrSnowflake"), description = "Path parameter that can take a [`Name`] or [`Snowflake`] identifier."),
    queryParameter("cursor", snowflake, description = "Cursor to passthrough to proceed into the next or previous page."),
    queryParameter("per_page", int32, description = "How many elements should be present in a page."),
    queryParameter("order", schema!("OrderBy"), description = "Order to sort the entries by.")
)]
pub async fn list_user_repositories(
    State(Instance { controllers, .. }): State<Instance>,
    NameOrSnowflake(nos): NameOrSnowflake,
    Query(PaginationQuery {
        mut per_page,
        cursor,
        order,
    }): Query<PaginationQuery>,
    session: Option<Extension<Session>>,
) -> Result<Pagination<Repository>> {
    let owner = match controllers.users.get_by(&nos).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(err(
                StatusCode::NOT_FOUND,
                (
                    ErrorCode::EntityNotFound,
                    "user with id or name doesn't exist",
                    json!({"idOrName":nos}),
                ),
            ))
        }

        Err(_) => return Err(internal_server_error()),
    };

    let list_private_stuff = match session {
        Some(Extension(Session { user, .. })) => owner.id == user.id,
        None => false,
    };

    if per_page > 100 {
        return Err(err(
            StatusCode::NOT_ACCEPTABLE,
            (
                ErrorCode::MaxPerPageExceeded,
                "`per_page` query parameter can't go over 100 entries",
                json!({"perPage": per_page}),
            ),
        ));
    }

    per_page = cmp::min(10, per_page);
    controllers
        .repositories
        .paginate(PaginationRequest {
            list_private_stuff,
            owner_id: Some(owner.id.try_into().unwrap()),
            order_by: order,
            per_page,
            cursor,
            metadata: azalia::hashmap!(),
        })
        .await
        .map(|data| ok(StatusCode::OK, data))
        .map_err(|_| internal_server_error())
}

/// Create a repository with the current authenticated user as the owner of the repository
#[controller(
    method = put,
    tags("Repositories"),
    requestBody("Payload for creating a repository", ("application/json", schema!("CreateRepositoryPayload"))),
    response(201, "Repository created", ("application/json", response!("RepositoryResponse"))),
    response(400, "Bad Request", ("application/json", response!("ApiErrorResponse"))),
    response(409, "Conflict: repository with that name already exists on the user's account", ("application/json", response!("ApiErrorResponse"))),
    response(500, "Internal Server Error", ("application/json", response!("ApiErrorResponse")))
)]
pub async fn create_user_repository(
    State(Instance {
        controllers,
        snowflake,
        pool,
        ..
    }): State<Instance>,
    Extension(Session { user, .. }): Extension<Session>,
    Json(payload): Json<CreateRepositoryPayload>,
) -> Result<Repository> {
    validate(&payload, CreateRepositoryPayload::validate)?;

    match sqlx::query_as::<Postgres, Repository>(
        "select repositories.id from repositories where name = $1 and owner = $2;",
    )
    .bind(&payload.name)
    .bind(user.id)
    .fetch_optional(&pool)
    .await
    {
        Ok(None) => {}
        Ok(Some(_)) => {
            return Err(err(
                StatusCode::CONFLICT,
                (
                    ErrorCode::EntityAlreadyExists,
                    "repository with given name already exists on your account",
                    json!({"name":payload.name}),
                ),
            ))
        }

        Err(e) => {
            error!(error = %e, user.id, %payload.name, "unable to find a user repository with name");
            sentry::capture_error(&e);

            return Err(internal_server_error());
        }
    }

    let id = snowflake.generate();
    let now = Local::now();
    let repo = Repository {
        description: payload.description.clone(),
        created_at: now,
        updated_at: now,
        private: payload.private,
        r#type: payload.r#type,
        owner: user.id,
        name: payload.name.clone(),
        id: i64::try_from(id.value()).unwrap(),

        ..Default::default()
    };

    controllers
        .repositories
        .create(payload, &repo)
        .await
        .map(|_| ok(StatusCode::CREATED, repo))
        .map_err(|_| internal_server_error())
}
*/
*/
