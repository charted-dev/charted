// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2025 Noelware, LLC. <team@noelware.org>
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

//! An atomic Ulid generator.
//!
//! The implementation is loosely based off from the [`ulid`'s Generator] but
//! can be used atomically without `&mut self`. Credit for the implementation
//! goes to them, not us.
//!
//! [`ulid`'s Generator]: https://github.com/dylanhart/ulid-rs/blob/master/src/generator.rs

use crate::api;
use rand::Rng;
use std::{
    ptr::null,
    sync::atomic::{AtomicPtr, Ordering},
    time::{Duration, SystemTime},
};
use ulid::Ulid;

/// A error when the ULID will overflow into the next millisecond
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("monotonic clock overflow in ulid generator?!")]
pub struct MonotonicTimeOverflow {
    _priv: (),
}

impl From<MonotonicTimeOverflow> for api::Error {
    fn from(_: MonotonicTimeOverflow) -> Self {
        api::Error::from((
            api::ErrorCode::SystemFailure,
            "monotonic clock failure when generating a new ulid",
        ))
    }
}

/// Atomic generator for ULIDs.
#[derive(Debug)]
pub struct Generator {
    previous: AtomicPtr<Ulid>,
}

impl Generator {
    /// Creates a new [`Generator`] object with the previous ULID
    /// pointing to a nil ULID.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Generator {
        Generator {
            previous: AtomicPtr::new(null::<Ulid>() as *mut Ulid),
        }
    }

    fn previous(&self) -> *mut Ulid {
        self.previous.load(Ordering::SeqCst)
    }

    /// Generate a new [`Ulid`] atomically with the current system time and the operating
    /// system's generator as the entropy source.
    #[track_caller]
    pub fn generate(&self) -> Result<Ulid, MonotonicTimeOverflow> {
        self.generate_with_time(SystemTime::now())
    }

    /// Generate a new [`Ulid`] atomically with a specified [`SystemTime`].
    #[track_caller]
    pub fn generate_with_time(&self, time: SystemTime) -> Result<Ulid, MonotonicTimeOverflow> {
        self._generate(time, &mut rand::rng())
    }

    // Credit for the implementation: https://github.com/dylanhart/ulid-rs/blob/b39ecb97c6a1e4dba34a67d38f12d97b7305ded1/src/generator.rs
    fn _generate<R: Rng + ?Sized>(&self, time: SystemTime, entropy: &mut R) -> Result<Ulid, MonotonicTimeOverflow> {
        // If it is a null ptr, then we will just generate it.
        let ulid = if self.previous().is_null() {
            &Ulid::nil()
        } else {
            // Safety: we checked that `self.previous()` is null and we are not
            // in that branch.
            unsafe { &*self.previous() }
        };

        let last_timestamp = ulid.timestamp_ms();

        // We are going to assume either time went backwards OR it is the same
        // millisecond timestamp. If so, then we should increment that it is
        // monotonic.
        if time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() <=
            u128::from(last_timestamp)
        {
            if let Some(mut next) = ulid.increment() {
                self.previous.store(&mut next, Ordering::SeqCst);
                return Ok(next);
            } else {
                return Err(MonotonicTimeOverflow { _priv: () });
            }
        }

        let mut next = Ulid::from_datetime_with_source(time, entropy);
        self.previous.store(&mut next, Ordering::SeqCst);

        Ok(next)
    }
}

impl Clone for Generator {
    fn clone(&self) -> Self {
        Generator {
            previous: AtomicPtr::new(self.previous.load(Ordering::SeqCst)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;
    use std::time::{Duration, SystemTime};
    use ulid::Ulid;

    fn __assert_send<S: Send>() {}
    fn __assert_sync<S: Sync>() {}

    #[test]
    fn test_monotonicity() {
        let now = SystemTime::now();
        let generator = Generator::new();

        let ulid1 = generator.generate_with_time(now).unwrap();
        let ulid2 = generator.generate_with_time(now).unwrap();
        let ulid3 = Ulid::from_datetime(now + Duration::from_millis(1));

        assert_eq!(ulid1.0 + 1, ulid2.0);
        assert!(ulid2 < ulid3);
        assert!(ulid2.timestamp_ms() < ulid3.timestamp_ms());
    }

    #[test]
    fn test_send_sync() {
        __assert_send::<Generator>();
        __assert_sync::<Generator>();
    }
}
