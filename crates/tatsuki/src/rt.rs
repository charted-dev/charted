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

//! Implements types and implementations for different asynchronous runtimes.
//!
//! * [`tokio`] is fully supported.
//!
//! [`tokio`]: https://tokio.rs

use std::{future::Future, pin::Pin, time::Duration};

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;

/// Represents an agnostic asynchronous runtime that can perform tasks that Tatsuki requires.
pub trait Runtime: Sized + Send + Sync {
    /// Spawns a [`Future`] onto a runtime.
    fn spawn<F: Future + Send + 'static>(&self, fut: F)
    where
        F::Output: Send + 'static;

    /// Return a [`Future`] that resolves in `duration` time.
    fn sleep(&self, duration: Duration) -> Pin<Box<dyn Sleep>>;
}

/// Future returned by [`Runtime::sleep`].
pub trait Sleep: Future<Output = ()> + Send + Sync {}
