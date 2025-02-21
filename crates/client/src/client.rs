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

use crate::{Error, Result};
use charted_core::api;
use futures_util::TryFutureExt;
use reqwest::{Body, ClientBuilder, Method, Response, header::HeaderMap};
use url::Url;

/// The default API endpoint.
pub const DEFAULT_API_ENDPOINT: &str = "https://charts.noelware.org/api/";

/// The default API version.
pub const DEFAULT_API_VERSION: api::Version = api::Version::V1;

/// The actual client to use when sending requests.
#[derive(Debug, Clone)]
pub struct Client {
    inner: reqwest::Client,
    base: Url,
}

impl Client {
    /// Creates a new [`Client`] instance with a base URL.
    pub fn new<U: TryInto<Url, Error = url::ParseError>>(base: U, version: api::Version) -> Result<Self> {
        let url: Url = base.try_into()?;
        let base = url.join(&format!("/{version}"))?;

        Ok(Client {
            inner: ClientBuilder::new().build()?,
            base,
        })
    }

    /// Replaces the default [`reqwest::Client`] with your own.
    pub fn with_client<C: Into<reqwest::Client>>(self, client: C) -> Self {
        Self {
            inner: client.into(),
            ..self
        }
    }

    #[doc(hidden)]
    #[cfg_attr(feature = "tracing", tracing::instrument(name = "charted.client.request", skip_all))]
    pub fn send<B: Into<Option<Body>>>(
        &self,
        method: Method,
        endpoint: impl Into<String>,
        headers: Option<HeaderMap>,
        body: B,
    ) -> impl Future<Output = Result<Response>> + Send {
        let endpoint = endpoint.into();

        #[cfg(feature = "tracing")]
        ::tracing::debug!("<- {} {}", method, endpoint);

        let mut builder = self.inner.request(method.clone(), self.base.join(&endpoint).unwrap());
        if let Some(headers) = headers {
            builder = builder.headers(headers);
        }

        if let Some(body) = body.into() {
            builder = builder.body(body);
        }

        builder
            .send()
            .inspect_ok(move |res| {
                #[cfg(feature = "tracing")]
                ::tracing::debug!(
                    "-> {} {}: {} (success: {})",
                    method,
                    endpoint,
                    res.status(),
                    res.status().is_success()
                );

                #[cfg(not(feature = "tracing"))]
                let _ = res;
            })
            .map_err(Error::Reqwest)
    }
}

/// The default implementation will use [`DEFAULT_API_ENDPOINT`] as the base.
impl Default for Client {
    fn default() -> Self {
        Self::new(DEFAULT_API_ENDPOINT, DEFAULT_API_VERSION).unwrap()
    }
}

impl From<reqwest::Client> for Client {
    fn from(value: reqwest::Client) -> Self {
        Self {
            inner: value,
            ..Default::default()
        }
    }
}
