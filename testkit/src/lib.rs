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

mod client;

use async_trait::async_trait;
use axum::Router;
use client::HttpClient;
use eyre::{eyre, Context, Result};
use hyper::Body;
use std::{borrow::Cow, future::Future, net::TcpListener};
use tokio::task::JoinHandle;

/// Represents a test context. A test context is a way to use different methods
/// on a test bed that allows execution over the underlying HTTP server.
#[derive(Debug)]
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
    pub fn new<C: Into<Cow<'static, str>>>(name: C) -> TestContext {
        TestContext {
            name: name.into(),
            http: HttpClient::new(Cow::Borrowed("http://localhost:0")),
            server_handle: None,
        }
    }

    pub fn start_server(&mut self, router: Router) -> Result<()> {
        if self.server_handle.is_some() {
            return Err(eyre!("test server is already listening"));
        }

        let listener = TcpListener::bind("127.0.0.1:0").context("unable to bind to ephemeral socket")?;
        let addr = listener
            .local_addr()
            .context("unable to get local address of ephemeral socket")?;

        self.http = HttpClient::new(Cow::Owned(addr.to_string()));
        self.server_handle = Some(tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(router.into_make_service())
                .await
                .context("hyper server error")
        }));

        Ok(())
    }
}

/// Represents a single test that can be invoked.
#[async_trait]
pub trait Test: Send + Sync {
    async fn invoke(&self, context: TestContext);
}

#[async_trait]
impl<F, Fut> Test for F
where
    F: Fn(TestContext) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send + Sync,
{
    async fn invoke(&self, context: TestContext) {
        (self)(context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing, Router};

    fn router() -> Router {
        Router::new().route("/", routing::get(|| async { "Hello, world!" }))
    }

    #[tokio::test]
    async fn test_context_example() {
        async fn my_test(context: TestContext) {
            let res = context.http.get("/").send().await.unwrap();
            let text = res.text().await.unwrap();

            assert_eq!(&text, "Hello, world!");
        }

        let mut ctx = TestContext::new("a test context example");
        ctx.start_server(router()).unwrap();

        // could've done my_text(ctx) but, this is the purpose
        // of testing the Test trait (and it works!)
        //
        // building block of the #[charted_server::test] macro :0
        (&my_test as &dyn Test).invoke(ctx).await;
    }
}
