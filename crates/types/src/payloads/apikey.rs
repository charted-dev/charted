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

use crate::name::Name;
use charted_core::serde::Duration;
use serde::Deserialize;
use utoipa::ToSchema;

super::create_modifying_payload! {
    ApiKey;

    /// Payload object for constructing an API key.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    create {
        /// Short description about the API key. Used to visibility distinct
        /// an API key other than its name.
        #[serde(default)]
        pub description: Option<String>,

        /// Maximum of time that this API key can live. Minimum allowed is 30 seconds.
        #[serde(default)]
        pub expires_in: Option<Duration>,

        /// Name of the API key.
        pub name: Name,
    }

    /// Payload object for modifying a API key.
    #[derive(Debug, Clone, Deserialize, ToSchema)]
    patch {
        /// Updates or removes the description of the API key.
        ///
        /// * If this is `null`, this will not do any patching
        /// * If this is a empty string, this will act as "removing" it from the metadata
        /// * If the comparsion (`old.description == this.description`) is false, then this will update it.
        #[serde(default)]
        pub description: Option<String>,

        /// key name to use to identify the key
        #[serde(default)]
        pub name: Option<Name>,
    }
}
