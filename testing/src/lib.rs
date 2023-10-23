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

mod client;

use async_trait::async_trait;
use client::HttpClient;
use eyre::{eyre, Context, Result};
use hyper::{body::HttpBody, Body, Request, Response, Server};
use std::{borrow::Cow, future::Future, net::TcpListener};
use tokio::task::JoinHandle;
use tower::make::Shared;
use tower_service::Service;

/// Represents a test context. A test context is a way to use different methods
/// on a test bed that allows execution over the underlying HTTP server.
pub struct TestContext {
    /// The name of the test that is running
    pub name: Cow<'static, str>,

    /// Represents the [`HttpClient`] that allows to send requests to.
    pub http: HttpClient<Body>,

    // the server handle goes last since it'll take some time for the Hyper
    // server to be destroyed.
    server_handle: Option<JoinHandle<Result<()>>>,
}

impl TestContext {
    /// Creates a new [`TestContext`] instance.
    pub fn new(name: Cow<'static, str>) -> TestContext {
        TestContext {
            name,
            http: HttpClient::new(Cow::Borrowed("http://localhost:0")),
            server_handle: None,
        }
    }

    /// Ignite the test context and launches the Axum server
    pub fn ignite<S, ResBody>(&mut self, svc: S) -> Result<()>
    where
        S: Service<Request<Body>, Response = Response<ResBody>> + Clone + Send + 'static,
        ResBody: HttpBody + Send + 'static,
        ResBody::Data: Send,
        ResBody::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
        S::Future: Send,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        if self.server_handle.is_some() {
            return Err(eyre!("test server is already listening"));
        }

        let listener = TcpListener::bind("127.0.0.1:0").context("unable to bind to ephemeral socket")?;
        let addr = listener
            .local_addr()
            .context("unable to get local address of ephemeral socket")?;

        self.http = HttpClient::new(Cow::Owned(addr.to_string()));
        self.server_handle = Some(tokio::spawn(async move {
            let server = Server::from_tcp(listener).unwrap().serve(Shared::new(svc));
            server.await.context("hyper server error")
        }));

        Ok(())
    }
}

/// Represents a single test that can be invoked.
#[async_trait]
pub trait Test: Send + Sync {
    /// Invokes the given test and returns a [`Result`] of the execution itself.
    async fn invoke(&self, context: TestContext) -> Result<()>;
}

#[async_trait]
impl<F, Fut> Test for F
where
    F: Fn(TestContext) -> Fut + Send + Sync,
    Fut: Future<Output = Result<()>> + Send + Sync,
{
    async fn invoke(&self, context: TestContext) -> Result<()> {
        (self)(context).await
    }
}
