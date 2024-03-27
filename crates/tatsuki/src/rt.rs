// 🐻‍❄️🪆 tatsuki: Dead simple job scheduling library
// Copyright (c) 2024 Noel Towa <cutie@floofy.dev>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Implements types and implementations for different asynchronous runtimes. [`tokio`] is fully supported
//! and [`async-std`] usage is experimental.
//!
//! [`tokio`]: https://tokio.rs
//! [`async-std`]: https://async.rs

use std::{future::Future, pin::Pin, time::Duration};

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;

#[cfg(feature = "async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-std")))]
pub mod async_std;

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
