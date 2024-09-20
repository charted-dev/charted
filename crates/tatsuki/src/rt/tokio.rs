// 🐻‍❄️🗻 tatsuki: Dead simple asynchronous job scheduler that is runtime-agnostic.
// Copyright 2024 Noel Towa <cutie@floofy.dev>
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

//! [`Runtime`] implementation using Tokio

use crate::rt::{self, Runtime};
use pin_project_lite::pin_project;
use std::{future::Future, pin::Pin};

/// [`Runtime`] implementation using the [`tokio`] library.
#[derive(Clone, Default)]
pub struct Tokio;

impl Runtime for Tokio {
    fn spawn<F: std::future::Future + Send + 'static>(&self, fut: F)
    where
        F::Output: Send + 'static,
    {
        tokio::task::spawn(fut);
    }

    fn sleep(&self, duration: std::time::Duration) -> Pin<Box<dyn rt::Sleep>> {
        Box::pin(Sleep {
            inner: tokio::time::sleep(duration),
        })
    }
}

pin_project! {
    pub(crate) struct Sleep {
        #[pin]
        pub(crate) inner: tokio::time::Sleep
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}

impl rt::Sleep for Sleep {}
