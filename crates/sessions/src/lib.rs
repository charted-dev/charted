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

mod manager;
mod provider;

pub use manager::*;
pub use provider::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub session_id: u64,
    pub user_id: u64,
}

impl Session {
    /// Returns a sanitized version of a [`Session`] that returns `None`
    /// on the `refresh_token` and `access_token` properties. This
    /// has to be used if querying sessions is going to be a thing.
    pub fn sanitized(self) -> Session {
        Session {
            session_id: self.session_id,
            user_id: self.user_id,

            ..Default::default()
        }
    }
}
