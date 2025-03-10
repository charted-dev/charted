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

use crate::openapi::ApiResponse;
use axum::http::StatusCode;
use charted_core::api;
use serde::Serialize;
use std::collections::BTreeMap;
use utoipa::{
    IntoResponses, ToResponse, ToSchema,
    openapi::{Ref, RefOr, Response},
};

/// A list of enabled features on this instance.
#[derive(Serialize, ToSchema, Default)]
pub struct Features {
    /// Whether if the [**Garbage Collection**] feature is enabled.
    ///
    /// As of this version, the **Garbage Collection** feature isn't implemented yet! You
    /// can track [the progress of the feature].
    ///
    /// [the progress of the feature]: #
    /// [**Garbage Collection**]: https://charts.noelware.org/docs/server/latest/features/garbage-collection
    pub garbage_collection: bool,

    /// Whether if the [**Audit Logging**] feature is enabled.
    ///
    /// As of this version, the **Audit Logging** feature isn't implemented yet! You
    /// can track [the progress of the feature].
    ///
    /// [the progress of the feature]: #
    /// [**Garbage Collection**]: https://charts.noelware.org/docs/server/latest/features/audit-logging
    pub audit_logs: bool,

    /// Whether if the [**HTTP Webhooks**] feature is enabled.
    ///
    /// As of this version, the **HTTP Webhooks** feature isn't implemented yet! You
    /// can track [the progress of the feature].
    ///
    /// [the progress of the feature]: #
    /// [**HTTP Webhooks**]: https://charts.noelware.org/docs/server/latest/features/webhooks
    pub webhooks: bool,

    /// Whether if the [**Search**] feature is enabled.
    ///
    /// As of this version, the **Search** feature isn't implemented yet! You
    /// can track [the progress of the feature].
    ///
    /// [the progress of the feature]: #
    /// [**Search**]: https://charts.noelware.org/docs/server/latest/features/garbage-collection
    pub search: bool,

    /// Whether if the [**TOTP (Time-based one time password)**] feature is enabled.
    ///
    /// As of this version, the **TOTP (Time-based one time password)** feature isn't
    /// implemented yet! You can track [the progress of the feature].
    ///
    /// [**TOTP (Time-based one time password)**]: https://charts.noelware.org/docs/server/latest/features/totp
    /// [the progress of the feature]: #
    pub totp: bool,

    /// Whether if the [**OCI Registry**] feature is enabled.
    ///
    /// As of this version, the **OCI Registry** feature isn't implemented yet! You
    /// can track [the progress of the feature].
    ///
    /// [**TOTP (Time-based one time password)**]: https://charts.noelware.org/docs/server/latest/features/oci
    /// [the progress of the feature]: #
    pub oci: bool,
}

impl IntoResponses for ApiResponse<Features> {
    fn responses() -> BTreeMap<String, RefOr<Response>> {
        azalia::btreemap!(
            "200" => RefOr::Ref(Ref::from_response_name(ApiResponse::<Features>::response().0))
        )
    }
}

/// Returns a list of enabled server features.
#[cfg_attr(debug_assertions, axum::debug_handler)]
#[utoipa::path(
    get,

    path = "/v1/features",
    operation_id = "features",
    tag = "Main",
    responses(ApiResponse<Features>)
)]
pub async fn features() -> api::Response<Features> {
    api::from_default(StatusCode::OK)
}
