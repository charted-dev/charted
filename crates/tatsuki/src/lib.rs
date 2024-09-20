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

#![doc(html_logo_url = "https://cdn.floofy.dev/images/August.png")]
#![doc = include_str!("../README.md")]
#![cfg_attr(any(docsrs, noeldoc), feature(doc_cfg))]
#![allow(rustdoc::broken_intra_doc_links)] // we use GitHub's alerts and rustdoc doesn't like them
#![allow(unused)]

use std::{any::Any, fmt::Debug, future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;

pub mod job;
pub mod rt;

mod clock;
pub use clock::*;

/// Type-alias that represents a boxed future.
pub type BoxedFuture<'a, Output> =
    ::core::pin::Pin<::std::boxed::Box<dyn ::core::future::Future<Output = Output> + Send + 'a>>;

pub struct Scheduler<R: rt::Runtime, C: Clock = clock::ChronoClock> {
    cancellation_token: CancellationToken,
    runtime: R,
    clock: C,
    jobs: Vec<Arc<dyn Any + Send + Sync>>,
}

impl<R: rt::Runtime + Debug, C: Clock + Debug> Debug for Scheduler<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scheduler")
            .field("cancelled", &self.cancellation_token.is_cancelled())
            .field("runtime", &self.runtime)
            .field("clock", &self.clock)
            .field("jobs", &self.jobs.len())
            .finish()
    }
}

/// Warning: cloning a [`Scheduler`] will create a sibling cancellation token, and if `.cancel` was called
/// from a parent cancellation token, the event loop will no longer trigger
impl<R: rt::Runtime + Clone, C: Clock + Clone> Clone for Scheduler<R, C> {
    fn clone(&self) -> Self {
        Self {
            cancellation_token: self.cancellation_token.clone(),
            runtime: self.runtime.clone(),
            clock: self.clock.clone(),
            jobs: self.jobs.clone(),
        }
    }
}

impl<R: rt::Runtime + Default, C: Clock + Default> Default for Scheduler<R, C> {
    fn default() -> Self {
        Self {
            cancellation_token: CancellationToken::default(),
            runtime: R::default(),
            clock: C::default(),
            jobs: Vec::new(),
        }
    }
}

impl<R: rt::Runtime + 'static, C: Clock + 'static> Scheduler<R, C> {
    /// Creates a new, empty [`Scheduler`].
    ///
    /// As this will require defining a runtime, you can use the `tokio` method
    /// in the crate to use the defined runtimes instead.
    pub fn new(runtime: R, clock: C) -> Scheduler<R, C> {
        Scheduler {
            cancellation_token: CancellationToken::default(),
            runtime,
            clock,
            jobs: Vec::new(),
        }
    }

    /// Cancels the [`Scheduler`] from being scheduled onto the runtime.
    ///
    /// This will kill all jobs that are waiting to be processed and will wait
    /// for jobs that are running to be killed.
    ///
    /// NOTE: Calling [`Scheduler::cancel`] is not an atomic operation! Read
    /// the [`CancellationToken::cancel`] documentation for more information.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Returns `true` if the [`Scheduler`] had been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancellation_token.is_cancelled()
    }

    /// Replace the clock from this scheduler with a new clock.
    pub fn with_clock<C2: Clock + 'static>(self, clock: C2) -> Scheduler<R, C2> {
        Scheduler {
            cancellation_token: self.cancellation_token,
            runtime: self.runtime,
            clock,
            jobs: self.jobs,
        }
    }

    /// Process a single job tick. This will process all jobs once.
    pub async fn tick(&mut self) {
        // // Even if we were cancelled, `tick` can be called in userland code.
        // if self.is_cancelled() {
        //     return;
        // }

        #[cfg(feature = "tracing")]
        ::tracing::trace!(
            "we have {} scheduled jobs, determining which ones are going to be executed...",
            self.jobs.len()
        );

        #[cfg(feature = "log")]
        ::log::trace!(
            "we have {} scheduled jobs, determining which ones are going to be executed...",
            self.jobs.len()
        );
    }

    async fn run_pending(&mut self) {}
}

impl<R: rt::Runtime + Clone + 'static, C: Clock + Clone + 'static> Scheduler<R, C> {
    /// Emit a new future that will be spawned in the background to process call job ticks
    /// per 500ms to determine what jobs are avaliable to be scheduled or need to be pushed
    /// back.
    pub fn schedule_in_background(&self) {
        let mut me = self.clone();
        self.runtime.spawn(async move {
            #[cfg(feature = "tracing")]
            ::tracing::trace!(
                "scheduler was told to be ran in the background for {} jobs",
                me.jobs.len()
            );

            #[cfg(feature = "log")]
            ::log::trace!(
                "scheduler was told to be ran in the background for {} jobs",
                me.jobs.len()
            );

            // Well, we should schedule all jobs for now
            me.tick().await;

            loop {
                tokio::select! {
                    // If we receive a cancellation, then we need to
                    // break out of the loop.
                    _ = me.cancellation_token.cancelled() => {
                        break;
                    }

                    // Otherwise, if we can sleep for ~500ms, then
                    // we can process another job tick.
                    _ = me.runtime.sleep(Duration::from_millis(500)) => {
                        me.tick().await;
                    }
                }
            }

            #[cfg(feature = "tracing")]
            ::tracing::trace!("scheduler cancelled its execution");

            #[cfg(feature = "log")]
            ::log::trace!("scheduler cancelled its execution");
        });
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
/// Creates a new [`Scheduler`] using the [`rt::tokio::Tokio`] runtime.
pub fn tokio() -> Scheduler<rt::tokio::Tokio> {
    Scheduler::default()
}

pub type JobFuture<'a> = Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + Sync + 'a>;

struct ScheduledJobs<'a> {
    futures: Vec<Option<Pin<JobFuture<'a>>>>,
}
