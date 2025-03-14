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

//! An atomically, monotonic ULID generator.
//!
//! The implementation of `Generator` is loosely based off ulid's `Generator`
//! but it uses atomic pointers so that `&mut self` isn't required. Credits
//! to the ulid-rs crate for the implementation.

use rand::{TryRngCore, rngs::OsRng};
use std::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
    time::{Duration, SystemTime},
};
use ulid::Ulid;

/// Error type that occurred when generating a ULID.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum Error {
    #[display("monotonic time overflow occurred")]
    MonotonicTimeOverflow,

    #[display("os rng failure: {}", _0)]
    Rng(<rand::rngs::OsRng as rand::TryRngCore>::Error),
}

/// An atomically, monotonic ULID generator.
#[derive(Debug)]
pub struct Generator {
    previous: AtomicPtr<Ulid>,
}

impl Clone for Generator {
    fn clone(&self) -> Self {
        Generator {
            previous: AtomicPtr::new(self.previous()),
        }
    }
}

impl Generator {
    /// Creates a new [`Generator`].
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Generator {
        Generator {
            previous: AtomicPtr::new(null_mut()),
        }
    }

    fn previous(&self) -> *mut Ulid {
        self.previous.load(Ordering::SeqCst)
    }

    /// Generates a new [`Ulid`] atomically with the current system time and the OS'
    /// random generator as the entropy source.
    pub fn generate(&self) -> Result<Ulid, Error> {
        self._generate(SystemTime::now(), &mut OsRng)
    }

    /// Generates a new [`Ulid`] atomically with a specified [`SystemTime`] and uses the
    /// operating system's random generator as the entropy source.
    pub fn generate_with_time(&self, time: SystemTime) -> Result<Ulid, Error> {
        self._generate(time, &mut OsRng)
    }

    /// Generates a new [`Ulid`] atomically with a specified [`SystemTime`] and uses the
    /// operating system's random generator as the entropy source.
    pub fn generate_with_source_and_time<R: TryRngCore + Clone>(
        &self,
        time: SystemTime,
        entropy: &mut R,
    ) -> Result<Ulid, Error> {
        self._generate(time, entropy)
    }

    // credit: https://github.com/dylanhart/ulid-rs/blob/b39ecb97c6a1e4dba34a67d38f12d97b7305ded1/src/generator.rs
    fn _generate<R: TryRngCore + Clone>(&self, time: SystemTime, entropy: &mut R) -> Result<Ulid, Error> {
        let ulid = if !self.previous().is_null() {
            unsafe { *self.previous() }
        } else {
            Ulid::nil()
        };

        // We are going to assume either time went backwards OR it is the same
        // millisecond timestamp. If so, then we should increment that it is
        // monotonic.
        let ts = ulid.timestamp_ms();
        if time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() <=
            u128::from(ts)
        {
            if let Some(mut next) = ulid.increment() {
                self.previous.store(&mut next, Ordering::SeqCst);
                return Ok(next);
            } else {
                return Err(Error::MonotonicTimeOverflow);
            }
        }

        let mut next = Ulid::from_datetime_with_source(time, &mut entropy.clone().unwrap_err());
        self.previous.store(&mut next, Ordering::SeqCst);

        Ok(next)
    }
}

const _: () = {
    static _CAN_BE_USED_IN_STATIC: Generator = Generator::new();
};

crate::assert_send_and_sync!(Generator);

#[cfg(test)]
mod tests {
    use super::Generator;
    use std::time::{Duration, SystemTime};
    use ulid::Ulid;

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
}
