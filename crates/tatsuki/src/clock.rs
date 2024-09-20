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

use chrono::{DateTime, TimeZone};

/// A clock to determine the current time for a job.
pub trait Clock: Send {
    /// Determines the current time based off a given timezone.
    fn now<Tz: TimeZone + Send + Sync>(&self, tz: &Tz) -> DateTime<Tz>;
}

/// A [`Clock`] implementation that uses chrono's [`Local::now`] function to determine
/// the current time.
#[derive(Clone, Default)]
pub struct ChronoClock;

impl Clock for ChronoClock {
    fn now<Tz: TimeZone + Send + Sync>(&self, tz: &Tz) -> DateTime<Tz> {
        chrono::Local::now().with_timezone(tz)
    }
}

/// A clock meant for testing the job scheduler.
#[derive(Clone)]
pub struct TestClock<Tz: TimeZone>(DateTime<Tz>);
impl<Tz: TimeZone + 'static> TestClock<Tz> {
    /// Creates a new [`TestClock`].
    ///
    /// ## Example
    /// ```
    /// # use tatsuki::{tokio, Scheduler, rt::Tokio, TestClock};
    /// # use chrono::DateTime;
    /// #
    /// let scheduler: Scheduler<Tokio, TestClock> = tokio()
    ///     .with_clock(TestClock::new(DateTime::MIN_UTC));
    /// ```
    pub fn new<DT: Into<DateTime<Tz>>>(dt: DT) -> TestClock<Tz> {
        TestClock(dt.into())
    }
}

impl<Tz: TimeZone + Send> Clock for TestClock<Tz>
where
    Tz::Offset: Send,
{
    fn now<Tz2: TimeZone + Send + Sync>(&self, tz: &Tz2) -> DateTime<Tz2> {
        self.0.with_timezone(tz)
    }
}
