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

#![doc(html_logo_url = "https://cdn.floofy.dev/images/August.png")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::broken_intra_doc_links)] // we use GitHub's alerts and rustdoc doesn't like them

pub mod jobs;
pub mod rt;

mod snapshot;
pub use snapshot::*;

use crate::jobs::Job;
use std::{
    any::Any,
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio_util::sync::CancellationToken;

/// Represents an event loop that will schedule jobs onto an async runtime. This is the main
/// thing you will use when using the Tatsuki library.
pub struct EventLoop<RT: rt::Runtime> {
    _warned_cancelled_loop: bool,
    cancellation_token: CancellationToken,
    runtime: RT,

    // we use `dyn Any` since we want to type erase the GAT that the `Job` trait
    // requires, in the `poll` method, it'll only check if it is `Job<Error = Box<dyn std::error::Error>>` and
    // run that.
    jobs: Vec<Arc<dyn Any + Send + Sync>>,
}

/// Warning: cloning a [`EventLoop`] will create a sibling cancellation token, and if `.cancel` was called
/// from a parent cancellation token, the event loop will no longer trigger
impl<RT: Clone + rt::Runtime> Clone for EventLoop<RT> {
    fn clone(&self) -> Self {
        Self {
            _warned_cancelled_loop: self._warned_cancelled_loop,
            cancellation_token: self.cancellation_token.clone(),
            runtime: self.runtime.clone(),
            jobs: self.jobs.clone(),
        }
    }
}

impl<RT: Default + rt::Runtime> Default for EventLoop<RT> {
    fn default() -> EventLoop<RT> {
        EventLoop {
            _warned_cancelled_loop: false,
            cancellation_token: CancellationToken::default(),
            runtime: RT::default(),
            jobs: Vec::default(),
        }
    }
}

impl<RT: Clone + rt::Runtime + 'static> EventLoop<RT> {
    /// Creates a new, empty [`EventLoop`]. As this will require defining a runtime,
    /// you can use the [`tokio`] or [`async_std`] methods in the crate to use the
    /// defined runtimes instead.
    pub fn new(runtime: RT) -> EventLoop<RT> {
        EventLoop {
            _warned_cancelled_loop: false,
            cancellation_token: CancellationToken::default(),
            runtime,
            jobs: Vec::new(),
        }
    }

    /// Inserts a new job into the event loop.
    pub fn new_job<E: Into<Box<dyn std::error::Error>>, J: Job<Error = E> + 'static>(&mut self, job: J) {
        self.jobs.push(Arc::new(job));
    }

    /// Retain a snapshot of the event loop's execution.
    ///
    /// ## Checking if the event loop was cancelled
    /// We do not recommend doing:
    ///
    /// ```
    /// # let scheduler: tatsuki::EventLoop<_> = tatsuki::tokio();
    /// #
    /// // `scheduler` should be a EventLoop
    /// if scheduler.snapshot().cancelled {
    ///     // do things here i guess
    /// }
    /// ```
    ///
    /// to check whenever a [`EventLoop`] is cancelled, please use [`EventLoop::is_cancelled`] instead:
    ///
    /// ```
    /// # let scheduler: tatsuki::EventLoop<_> = tatsuki::tokio();
    /// #
    /// if scheduler.is_cancelled() {
    ///     // do things here!
    /// }
    /// ```
    ///
    /// Since a [`Snapshot`] will take some time to process all job execution, retaining a new
    /// snapshot everytime you need to check if a [`EventLoop`] is cancelled will take some time. Using
    /// the [`EventLoop::is_cancelled`] is better.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            snapshot_date: SystemTime::now(),
            cancelled: self.is_cancelled(),
            job_snapshots: HashMap::new(),
        }
    }

    /// Checks whenever if this [`EventLoop`] was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancellation_token.is_cancelled()
    }

    /// Cancels the [`EventLoop`] from being scheduled onto the runtime. This will kill
    /// all jobs that are waiting to be processed and will wait for jobs that are running
    /// to be killed.
    ///
    /// Do note that calling [`EventLoop::cancel`] is not an atomic operation!
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Process a single tick. This will only process all jobs once.
    pub async fn tick(&mut self) {
        if self.is_cancelled() {
            if !self._warned_cancelled_loop {
                #[cfg(feature = "tracing")]
                ::tracing::warn!("called #tick on a cancelled event loop, not doing anything");

                #[cfg(feature = "log")]
                ::log::warn!("called #tick on a cancelled event loop, will not be doing anything");

                self._warned_cancelled_loop = true;
            }

            return;
        }

        println!("called #tick :D");
    }

    /// Emit a new task spawned onto a runtime, which will call [`EventLoop::tick`] every
    /// 500ms.
    pub fn schedule(&self) {
        let mut us = self.clone();
        self.runtime.spawn(async move {
            #[cfg(feature = "tracing")]
            ::tracing::trace!("EventLoop was started execution of {} jobs", us.jobs.len());

            #[cfg(feature = "log")]
            ::log::warn!("EventLoop was started execution of {} jobs", us.jobs.len());

            loop {
                tokio::select! {
                    // once `EventLoop::cancel` is called, this will resolve
                    // and will break out of the loop and the task will
                    // succeed
                    _ = us.cancellation_token.cancelled() => {
                        break;
                    }

                    _ = us.runtime.sleep(Duration::from_millis(500)) => {
                        us.tick().await;
                    }
                }
            }

            #[cfg(feature = "tracing")]
            ::tracing::warn!("EventLoop was cancelled its execution");

            #[cfg(feature = "log")]
            ::log::warn!("EventLoop was cancelled its execution");
        });
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
/// Creates a new [`EventLoop`] using the [`rt::tokio::Tokio`] runtime.
pub fn tokio() -> EventLoop<rt::tokio::Tokio> {
    EventLoop::default()
}

#[cfg(feature = "async_std")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_std")))]
/// Creates a new [`EventLoop`] using the [`rt::async_std::AsyncStd`] runtime.
pub fn async_std() -> EventLoop<rt::async_std::AsyncStd> {
    EventLoop::default()
}
