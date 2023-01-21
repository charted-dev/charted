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

use std::collections::HashMap;

use clap::Parser;
use reqwest::{Body, Method};

use crate::api::generated_stub::User;
use crate::error::Error;
use crate::settings::Settings;

use super::AsyncExecute;

#[derive(Debug, Clone, Copy, Parser)]
#[command(about = "Returns the current user, if any")]
pub struct Me;

#[async_trait]
impl AsyncExecute for Me {
    async fn execute(self, settings: &Settings) -> Result<(), Error> {
        let keychain = settings.keychain();

        // 1. Check if we can access the os' keychain
        match keychain.get_api_key() {
            Ok(Some(key)) => {
                let user = get_user(settings, key).await?;

                // We should be fine since a error is logged.
                if user.is_none() {
                    return Ok(());
                }

                let user = user.unwrap();
                info!(
                    "Logged in as @{} ({}) on instance {}",
                    user.username,
                    user.id,
                    settings.server()
                );

                Ok(())
            }

            // 2. If not, we can inform the user that we don't have an valid
            // api key and we need to bark in the chat.
            Ok(None) => {
                warn!("There is no API key attached to instance [{}], use the `helm charted login` command to create an api key for that instance.", settings.server());
                Ok(())
            }

            Err(e) => return Err(Error::Unknown(Box::new(e))),
        }
    }
}

async fn get_user(settings: &Settings, api_key: String) -> Result<Option<User>, Error> {
    let headers = {
        let mut h = HashMap::new();
        h.insert("Authorization".to_string(), format!("ApiKey {api_key}"));

        h
    };

    settings
        .client()
        .request::<serde_json::Value, Body, &str>(Method::GET, "/users/@me", None, Some(headers))
        .await
        .map(|f| {
            // `success` is always present, so it's fine to unwrap
            let success = f["success"].as_bool().unwrap();
            if success {
                let data = &f["data"];
                Some(serde_json::from_value(data.to_owned()).unwrap())
            } else {
                error!(
                    "API server returned `success` = false, did we fuck up?\n{:?}",
                    &f["errors"]
                );
                None
            }
        })
}
