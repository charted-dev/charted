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

use axum::http::{HeaderName, HeaderValue};
use hyper::{
    body::HttpBody,
    client::{Builder, HttpConnector},
    Client, HeaderMap, Method, Request, Response, Uri,
};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, error::Error, fmt::Display, str::FromStr};

/// Represents a generic HTTP client that is an abstraction over [`hyper::Client`].
#[derive(Debug, Clone)]
pub struct HttpClient<B: HttpBody> {
    base_url: Cow<'static, str>,
    inner: Client<HttpConnector, B>,
}

impl<B: HttpBody + Default + Send + 'static> HttpClient<B>
where
    <B as HttpBody>::Data: Send,
    <B as HttpBody>::Error: Into<Box<dyn Error + Send + Sync + 'static>>,
{
    /// Creates a new [`HttpClient`] instance.
    pub fn new(base_url: Cow<'static, str>) -> Self {
        Self {
            base_url,
            inner: Builder::default().build_http(),
        }
    }

    /// Creates a HTTP request builder for a GET request
    pub fn get<I: Display + Send>(&self, url: I) -> HttpRequest<B> {
        HttpRequest::new(self.base_url.clone(), self.inner.clone(), Method::GET, url)
    }

    /// Creates a HTTP request builder for a PUT request
    pub fn put<I: Display + Send>(&self, url: I) -> HttpRequest<B> {
        HttpRequest::new(self.base_url.clone(), self.inner.clone(), Method::PUT, url)
    }

    /// Creates a HTTP request builder for a POST request
    pub fn post<I: Display + Send>(&self, url: I) -> HttpRequest<B> {
        HttpRequest::new(self.base_url.clone(), self.inner.clone(), Method::POST, url)
    }

    /// Creates a HTTP request builder for a PATCH request
    pub fn patch<I: Display + Send>(&self, url: I) -> HttpRequest<B> {
        HttpRequest::new(self.base_url.clone(), self.inner.clone(), Method::PATCH, url)
    }

    /// Creates a HTTP request builder for a DELETE request
    pub fn delete<I: Display + Send>(&self, url: I) -> HttpRequest<B> {
        HttpRequest::new(self.base_url.clone(), self.inner.clone(), Method::DELETE, url)
    }
}

/// Represents a HTTP request loosely based off [`hyper::Request`] that
/// is just a builder to send a request.
#[derive(Debug)]
pub struct HttpRequest<B: HttpBody + Send> {
    client: Client<HttpConnector, B>,
    method: Method,
    inner: Request<B>,
}

impl<B: HttpBody + Default + Send + 'static> HttpRequest<B>
where
    <B as HttpBody>::Data: Send,
    <B as HttpBody>::Error: Into<Box<dyn Error + Send + Sync + 'static>>,
{
    fn new<U: Display>(base_url: Cow<'static, str>, client: Client<HttpConnector, B>, method: Method, url: U) -> Self {
        let full_url = format!("{}{}", base_url, url);
        let mut req = Request::new(B::default());

        *req.uri_mut() = Uri::from_str(&full_url).unwrap();
        let _ = req.headers_mut().insert(
            HeaderName::from_static("user-agent"),
            HeaderValue::from_static(
                "Noelware/charted-server ~ integration test suite (+https://github.com/charted-dev/charted)",
            ),
        );

        Self {
            client,
            method,
            inner: req,
        }
    }

    /// Inserts a single header into this request
    pub fn header<K: Into<HeaderName>, V: Into<HeaderValue>>(mut self, header: K, value: V) -> Self {
        let _ = self.inner.headers_mut().insert(header.into(), value.into());
        self
    }

    /// Override the [`HeaderMap`] in a given request.
    pub fn headers(mut self, map: HeaderMap) -> Self {
        *self.inner.headers_mut() = map;
        self
    }

    /// Sets the request body to `B`. This will panic if this is a GET
    /// or HEAD request.
    pub fn body(mut self, body: B) -> Self {
        if self.method == Method::GET || self.method == Method::HEAD {
            panic!("you're not allowed to set a HTTP body for GET or HEAD requests");
        }

        *self.inner.body_mut() = body;
        self
    }

    /// Sends the request and returns a [`Result`] of the response.
    pub async fn send(self) -> Result<HttpResponse, hyper::Error> {
        self.client.request(self.inner).await.map(HttpResponse)
    }
}

/// Represents an abstraction over [`hyper::Response`] that allows to deserialize
/// the response body into different types (like strings, `T`, etc.)
#[derive(Debug)]
pub struct HttpResponse(Response<hyper::Body>);
impl HttpResponse {
    /// Asynchronously translate the response body into `D`. This method does panic
    /// if the body couldn't be translated into a [`Bytes`] container.
    pub async fn json<D: DeserializeOwned>(self) -> Result<D, serde_json::Error> {
        let bytes = hyper::body::to_bytes(self.0).await.unwrap();
        serde_json::from_slice(&bytes)
    }

    /// Asynchronously translate the response body into UTF-8 encoded text.
    pub async fn text(self) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = hyper::body::to_bytes(self.0).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}
