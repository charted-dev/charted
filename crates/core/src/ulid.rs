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

use crate::api;
use rand::{rngs::OsRng, Rng};
use std::{
    borrow::Cow,
    fmt::Display,
    sync::atomic::{AtomicPtr, Ordering},
    time::{Duration, SystemTime},
};
use ulid::Ulid;

/// [`Error`] when the monotonic clock overflows
#[derive(Debug)]
pub struct MonotonicTimeOverflow {
    _priv: (),
}

impl Display for MonotonicTimeOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("montonic clock overflow?!")
    }
}

impl std::error::Error for MonotonicTimeOverflow {}
impl From<MonotonicTimeOverflow> for api::Error {
    fn from(_: MonotonicTimeOverflow) -> Self {
        api::Error {
            code: api::ErrorCode::SystemFailure,
            message: Cow::Borrowed("monotonic clock when generating ulids overflowed?!"),
            details: None,
        }
    }
}

/// An atomic Ulid generator.
#[derive(Debug)]
pub struct AtomicGenerator {
    previous_ulid: AtomicPtr<Ulid>,
}

impl Clone for AtomicGenerator {
    fn clone(&self) -> Self {
        AtomicGenerator {
            previous_ulid: AtomicPtr::new(self.previous_ulid.load(Ordering::SeqCst)),
        }
    }
}

impl AtomicGenerator {
    /// Creates a new [`AtomicGenerator`] instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> AtomicGenerator {
        AtomicGenerator {
            previous_ulid: AtomicPtr::new(&mut Ulid::nil()),
        }
    }

    pub(self) fn previous(&self) -> *mut Ulid {
        self.previous_ulid.load(Ordering::SeqCst)
    }

    /// Generates a new [`Ulid`] atomically with the current system time and the OS' random generator
    /// as the entropy source.
    pub fn generate(&self) -> Result<Ulid, MonotonicTimeOverflow> {
        self._generate(SystemTime::now(), &mut OsRng)
    }

    /// Generates a new [`Ulid`] atomically with a specified [`SystemTime`] and uses the operating
    /// system's random generator as the entropy source.
    pub fn generate_with_time(&self, time: SystemTime) -> Result<Ulid, MonotonicTimeOverflow> {
        self._generate(time, &mut OsRng)
    }

    // Credit for the implementation: https://github.com/dylanhart/ulid-rs/blob/05499bd1609f9ac4dd5f39e0bbf70da529179aed/src/generator.rs
    fn _generate<R: Rng + ?Sized>(&self, time: SystemTime, entropy: &mut R) -> Result<Ulid, MonotonicTimeOverflow> {
        // Safety: we ensured that all the points in the Safety documentation for
        //         `AtomicPtr::<T>::as_ref_unchecked()` are true:
        //
        //         * `Ulid::nil()` will resolve in a valid ptr
        //         * The previous ULID constructed from `AtomicGenerator::new()` is not a null pointer.
        let ulid = unsafe { self.previous().as_ref_unchecked() };
        let last_timestamp = ulid.timestamp_ms();

        // We are going to assume either time went backwards OR it is the same
        // millisecond timestamp. If so, then we should increment that it is
        // monotonic.
        if time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis()
            <= u128::from(last_timestamp)
        {
            if let Some(mut next) = ulid.increment() {
                self.previous_ulid.store(&mut next, Ordering::SeqCst);
                return Ok(next);
            } else {
                return Err(MonotonicTimeOverflow { _priv: () });
            }
        }

        let mut next = Ulid::from_datetime_with_source(time, entropy);
        self.previous_ulid.store(&mut next, Ordering::SeqCst);

        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicGenerator;
    use std::time::{Duration, SystemTime};
    use ulid::Ulid;

    fn __assert_send<S: Send>() {}
    fn __assert_sync<S: Sync>() {}

    #[test]
    fn test_monotonicity() {
        let now = SystemTime::now();
        let generator = AtomicGenerator::new();

        let ulid1 = generator.generate_with_time(now).unwrap();
        let ulid2 = generator.generate_with_time(now).unwrap();
        let ulid3 = Ulid::from_datetime(now + Duration::from_millis(1));

        assert_eq!(ulid1.0 + 1, ulid2.0);
        assert!(ulid2 < ulid3);
        assert!(ulid2.timestamp_ms() < ulid3.timestamp_ms());
    }

    #[test]
    fn test_send_sync() {
        __assert_send::<AtomicGenerator>();
        __assert_sync::<AtomicGenerator>();
    }
}
