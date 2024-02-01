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

mod manager;
pub use manager::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct Session {
    /// Refresh token. This will always be `null` if queried, but always will
    /// be present if you successfully logged in.
    pub refresh_token: Option<String>,

    /// Access token. This will always be `null` if queried, but always will
    /// be present if you successfully logged in.
    pub access_token: Option<String>,

    /// UUID of the session.
    #[schema(format = Uuid)]
    pub session: Uuid,

    /// ID of the user that created this session.
    pub user: u64,
}

impl Session {
    /// Returns a sanitized version of a [`Session`] that returns `None`
    /// on the `refresh_token` and `access_token` properties. This
    /// has to be used if querying sessions is going to be a thing.
    pub fn sanitized(self) -> Session {
        Session {
            session: self.session,
            user: self.user,

            ..Default::default()
        }
    }
}
