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

use std::{
    collections::{hash_map::Iter, HashMap},
    time::{Instant, SystemTime},
};

/// Represents a snapshot of the current execution pool of a [`EventLoop`][crate::EventLoop].
#[derive(Debug)]
pub struct Snapshot {
    /// time in unix milliseconds on when this snapshot was captured
    pub snapshot_date: SystemTime,

    /// whether or not the event loop is cancelled
    pub cancelled: bool,

    pub(crate) job_snapshots: HashMap<&'static str, JobSnapshot>,
}

impl Snapshot {
    /// Returns a snapshot of a single job.
    pub fn job<S: AsRef<str>>(&self, name: S) -> Option<JobSnapshot> {
        self.job_snapshots.get(name.as_ref()).cloned()
    }

    /// Returns a iterator of all the job snapshots a [`EventLoop`][crate::EventLoop] has collected.
    pub fn jobs(&self) -> Iter<'_, &str, JobSnapshot> {
        self.job_snapshots.iter()
    }
}

/// Represents a singular snapshot of a job's execution point.
#[derive(Debug, Clone)]
pub struct JobSnapshot {
    /// Returns a optional [`Instant`] refernece on when the last execution point
    /// for this job has concluded.
    pub last_delay_execution: Option<Instant>,

    /// The name of the job that this snapshot belongs to
    pub name: &'static str,
}
