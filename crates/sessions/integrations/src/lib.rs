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

use async_trait::async_trait;
use charted_sessions::Session;
use eyre::Result;
use serde::de::DeserializeOwned;

/// Represents an integration for authenticating users, but not authorizing users. This
/// is only meant to authenticate users from different platforms like [GitHub](https://github.com),
/// but not provide any authorization.
#[async_trait]
pub trait SessionIntegration: Send + Sync {
    /// The body that will be deserialized from the request.
    type Body: DeserializeOwned;

    /// Returns the name of this integration
    fn name(&self) -> &'static str;

    /// Returns an redirect URI.
    fn redirect_uri(&self) -> String;

    /// The actual code to run the callback that will return a [`Session`] out of it.
    async fn callback(&self, body: Self::Body) -> Result<Session>;
}
