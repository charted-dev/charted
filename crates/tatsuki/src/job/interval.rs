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

use super::Job;
use std::{borrow::Cow, error::Error, future::Future, marker::PhantomData, time::Duration};

/// A `Job` that will run on each interval (i.e, 30s).
pub struct IntervalBasedJob<
    'fut,
    F: FnOnce() -> Fut + Copy + Send + Sync,
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + Sync + 'fut,
> {
    __lt_marker: PhantomData<&'fut ()>,

    interval: Duration,
    name: Cow<'static, str>,
    f: F,
}

impl<'fut, F, Fut> IntervalBasedJob<'fut, F, Fut>
where
    F: FnOnce() -> Fut + Copy + Send + Sync,
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + Sync + 'fut,
{
    /// Creates a new [`IntervalBasedJob`] with a given function and duration.
    pub fn new(f: F, duration: Duration) -> Self {
        let name = std::any::type_name::<F>();

        IntervalBasedJob {
            __lt_marker: PhantomData,
            interval: duration,
            name: Cow::Borrowed(name),
            f,
        }
    }

    /// Overwrites the job name.
    ///
    /// By default, it'll use the [`type_name`][std::any::type_name] function from the
    /// standard library to determine the name of the job.
    pub fn with_name<S: Into<Cow<'static, str>>>(self, name: S) -> Self {
        Self {
            name: name.into(),

            ..self
        }
    }
}

impl<'fut, F, Fut> Job for IntervalBasedJob<'fut, F, Fut>
where
    F: FnOnce() -> Fut + Copy + Send + Sync,
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + Sync + 'fut,
{
    fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn can_be_executed(&self) -> bool {
        false
    }

    fn run(&mut self) -> crate::BoxedFuture<Result<(), Box<dyn Error>>> {
        Box::pin((self.f)())
    }
}

impl<'fut, F, Fut> From<(F, Duration)> for IntervalBasedJob<'fut, F, Fut>
where
    F: FnOnce() -> Fut + Copy + Send + Sync,
    Fut: Future<Output = Result<(), Box<dyn Error>>> + Send + Sync + 'fut,
{
    fn from((f, duration): (F, Duration)) -> Self {
        IntervalBasedJob::new(f, duration)
    }
}

#[cfg(test)]
#[test]
fn assert_send_sync() {
    fn __assert_send<S: Send>(_: &S) {}
    fn __assert_sync<S: Sync>(_: &S) {}

    async fn weow() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    let job = IntervalBasedJob::new(weow, Duration::from_secs(30));

    __assert_send(&job);
    __assert_sync(&job);
}

#[cfg(test)]
#[test]
fn assert_any_fn_can_work() {
    async fn weow() -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    let _ = IntervalBasedJob {
        __lt_marker: PhantomData,

        interval: Duration::from_secs(30),
        name: Cow::Borrowed("hello world"),
        f: weow,
    };
}
