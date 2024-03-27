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

// use super::Job;
// use async_trait::async_trait;
// use chrono::{TimeZone, Utc};
// use cron::{OwnedScheduleIterator, Schedule};
// use std::sync::{Arc, Mutex};

// /// Represents a [`Job`][super::Job] that uses cron expressions to determine
// /// how to process this job in numeric time.
// pub struct CronJob<Tz: TimeZone, Error = Box<dyn std::error::Error>> {
//     _has_panicd_before: bool,
//     iter: OwnedScheduleIterator<Tz>,
//     inner: Arc<Mutex<Box<dyn Job<Error = Error>>>>,
// }

// impl<Tz: TimeZone> CronJob<Tz> {
//     /// Creates a new [`CronJob`].
//     pub fn new<Err>(expr: Schedule, timezone: Tz, job: impl Job<Error = Err> + 'static) -> CronJob<Tz, Err> {
//         CronJob {
//             _has_panicd_before: false,
//             iter: expr.after_owned({
//                 let tz = timezone.clone();
//                 tz.from_utc_datetime(&Utc::now().naive_utc())
//             }),
//             inner: Arc::new(Mutex::new(Box::new(job))),
//         }
//     }
// }

// #[async_trait]
// impl<Tz: TimeZone + Send + Sync, Err: Into<Box<dyn std::error::Error>> + Send + Sync + 'static> Job for CronJob<Tz, Err>
// where
//     <Tz as chrono::TimeZone>::Offset: Send + Sync,
// {
//     type Error = Err;

//     async fn execute(&mut self) -> Result<(), Self::Error> {
//         if self._has_panicd_before {
//             return Ok(());
//         }

//         if let Some(next) = self.iter.next() {
//             dbg!(&next);
//         }

//         match self.inner.try_lock() {
//             Ok(_job) => Ok(()),
//             Err(_e) => {
//                 if !self._has_panicd_before {
//                     #[cfg(feature = "tracing")]
//                     ::tracing::error!(error = %_e, "received panic from inner job, will not try to schedule");

//                     self._has_panicd_before = true;
//                 }

//                 Ok(())
//             }
//         }
//     }
// }
