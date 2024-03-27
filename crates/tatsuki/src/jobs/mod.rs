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

//! Represents types for jobs that Tatsuki can schedule.

#[cfg(feature = "cron")]
#[cfg_attr(docsrs, doc(cfg(feature = "cron")))]
pub mod cron;

use async_trait::async_trait;
use std::future::Future;

use crate::JobSnapshot;

/// Represents a job that can be scheduled onto a Tatsuki event loop.
#[async_trait]
pub trait Job: Send + Sync {
    /// Error variant on when a job fails.
    type Error;

    /// Executes the job.
    async fn execute(&mut self) -> Result<(), Self::Error>;

    fn take_snapshot(&self) -> JobSnapshot;
}

#[async_trait]
impl<F, Fut, E> Job for F
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Result<(), E>> + Send + Sync,
{
    type Error = E;

    async fn execute(&mut self) -> Result<(), E> {
        (self)().await
    }

    fn take_snapshot(&self) -> JobSnapshot {
        unimplemented!("JobSnapshot cannot be taken for a function method as a job")
    }
}
