// 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
// Copyright 2022-2023 Noelware <team@noelware.org>
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

use reqwest::{Body, Method};

use crate::error::Error;

use super::client::Client;

#[derive(Debug, Clone)]
pub struct ApiKeys(Client);

impl ApiKeys {
    pub(crate) fn new(client: Client) -> ApiKeys {
        ApiKeys(client)
    }

    pub async fn get(
        &self,
        name: Option<String>,
    ) -> Result<Option<crate::api::generated_stub::ApiKeys>, Error> {
        let route = match name {
            Some(o) => format!("/apikeys/{o}"),
            None => "/apikeys".to_string(),
        };

        self.0
            .request::<Option<crate::api::generated_stub::ApiKeys>, Body, &str>(
                Method::GET,
                &route,
                None,
                None,
            )
            .await
    }
}
