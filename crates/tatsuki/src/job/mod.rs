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

use crate::BoxedFuture;
use std::{borrow::Cow, error::Error};

#[cfg(feature = "cron")]
mod cron;

#[cfg(feature = "cron")]
pub use cron::*;

mod interval;
pub use interval::*;

mod oneshot;
pub use oneshot::*;

mod schedule;
pub use schedule::*;

pub trait Job: Send + Sync {
    /// Returns the name of this job.
    fn name(&self) -> Cow<'static, str>;

    /// Checks whenever if we can be executed.
    fn can_be_executed(&self) -> bool;

    /// Runs the actual job.
    fn run(&mut self) -> BoxedFuture<Result<(), Box<dyn Error>>>;
}
