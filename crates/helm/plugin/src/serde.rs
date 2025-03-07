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

/// [serde] support for [`SecretString`](secrecy::SecretString)
pub mod secret_string {
    use secrecy::{ExposeSecret, SecretString};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(ss: &SecretString, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(ss.expose_secret())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<SecretString, D::Error> {
        Ok(SecretString::new(String::deserialize(deserializer)?.into()))
    }
}
