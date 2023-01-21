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

use std::{collections::HashMap, fmt::Debug, str::FromStr};

use crate::{error::Error, COMMIT_HASH, VERSION};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Body, Client as RClient, Method,
};
use serde::de::DeserializeOwned;

use super::apikeys::ApiKeys;

/// Represents an API client that connects to a `charted-server` instance to do API-related things. >:3
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    inner: RClient,
}

impl Client {
    pub fn new(server_url: &str, verbose: bool) -> Client {
        Client {
            base_url: server_url.to_string(),
            inner: RClient::builder()
                .connection_verbose(verbose)
                .http1_only() // charted-server doesn't support HTTP/2
                .user_agent(format!(
                    "Noelware/charted-helm v{}+{COMMIT_HASH}",
                    VERSION.trim()
                ))
                .build()
                .unwrap(),
        }
    }

    pub fn api_keys(self) -> ApiKeys {
        ApiKeys::new(self)
    }

    pub async fn request<U: DeserializeOwned + Debug, T: Into<Body>, E: AsRef<str>>(
        &self,
        method: Method,
        endpoint: E,
        body: Option<T>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<U, Error> {
        trace!(
            "<- {} {}",
            method.clone(),
            format!("{}{}", self.base_url, endpoint.as_ref())
        );

        let request = self.inner.request(
            method.clone(),
            format!("{}{}", self.base_url, endpoint.as_ref()),
        );

        if let Some(h) = headers {
            let mut map = HeaderMap::new();
            for (key, value) in h {
                map.insert(
                    HeaderName::from_str(key.clone().as_str())
                        .map_err(|e| Error::Unknown(Box::new(e)))?,
                    HeaderValue::from_str(value.clone().as_str())
                        .map_err(|e| Error::Unknown(Box::new(e)))?,
                );
            }

            let _ = request.try_clone().map(|f| f.headers(map));
        }

        if let Some(b) = body {
            let _ = request.try_clone().map(|f| f.body::<T>(b));
        }

        let req = request.build().map_err(|e| Error::Unknown(Box::new(e)))?;
        match self.inner.execute(req).await {
            Ok(res) if res.status().is_success() => {
                let status = res.status();
                let slice: &[u8] = &res.bytes().await.map_err(|e| Error::Unknown(Box::new(e)))?;
                let output: U =
                    serde_json::from_slice(slice).map_err(|e| Error::Unknown(Box::new(e)))?;

                trace!(
                    "<- {} {} -> {status}",
                    method.clone(),
                    format!("{}{}", self.base_url, endpoint.as_ref())
                );

                trace!("response body: {:?}", &output);
                Ok(output)
            }

            Ok(res) => {
                error!(
                    "<- {} {} -> {}",
                    method,
                    format!("{}{}", self.base_url, endpoint.as_ref()),
                    res.status()
                );

                let status = res.status();
                let slice: &[u8] = &res.bytes().await.map_err(|e| Error::Unknown(Box::new(e)))?;
                let s =
                    String::from_utf8(slice.to_vec()).map_err(|e| Error::Unknown(Box::new(e)))?;

                Err(Error::HttpRequest { status, body: s })
            }

            Err(e) => Err(Error::Unknown(Box::new(e))),
        }
    }
}
