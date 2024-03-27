// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2024 Noelware, LLC. <team@noelware.org>
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

use std::{cmp::min, collections::HashMap};

/// Represents a data structure that allows to interact with bits that act as a field
/// of bits. charted-server uses a bitfield structure to handle permissions like API key
/// scopes and repository/organization member permissions.
///
/// A bitfield can store a map of flags that are a indirection to a human-readable
/// name of that bit. So, we can map `repo:access` to `1 << 31`, which will mean
/// that `repo:access` is `1 << 31`.
#[derive(Debug, Clone)]
pub struct Bitfield {
    flags: HashMap<&'static str, u64>,
    value: u64,
}

impl Default for Bitfield {
    fn default() -> Self {
        Bitfield {
            flags: azalia::hashmap!(),
            value: 0,
        }
    }
}

impl Bitfield {
    pub fn new(value: u64, flags: HashMap<&'static str, u64>) -> Bitfield {
        Bitfield { flags, value }
    }

    /// Creates a new [`Bitfield`] data structure with a pre-determined map of flags.
    pub fn with_flags(flags: HashMap<&'static str, u64>) -> Bitfield {
        Bitfield { flags, value: 0 }
    }

    /// Sets a bit value for this [`Bitfield`]
    pub fn with_value(self, bits: u64) -> Bitfield {
        Bitfield { value: bits, ..self }
    }

    /// Returns a reference of all the flags available.
    pub fn flags(&self) -> &HashMap<&'static str, u64> {
        &self.flags
    }

    /// Returns a copied value of the bits that are used in this [Bitfield].
    pub fn bits(&self) -> u64 {
        self.value
    }

    /// Acculumate the maximum bit from the map of flags present.
    pub fn max(&self) -> u64 {
        self.flags.values().fold(0, |acc, curr| acc | curr)
    }

    /// Adds a set of bits into this [`Bitfield`], if any bits are over [`u64::MAX`] or over
    /// the acculumated [`max`][Bitfield::max], then it'll be ignored.
    pub fn add<I: IntoIterator<Item = u64>>(&mut self, bits: I) {
        let mut bits = bits.into_iter();
        let first = bits.next();
        if first.is_none() {
            return;
        }

        let mut additional = 0u64;
        additional |= first.unwrap();

        let max = self.max();
        for bit in bits {
            if bit == u64::MAX {
                continue;
            }

            if bit > max {
                continue;
            }

            additional |= bit;
        }

        self.value |= additional;
    }

    /// Adds a bit from a flag.
    pub fn from_flag<S: AsRef<str>>(&mut self, flag: S) {
        if let Some(flag) = self.flags.get(flag.as_ref()) {
            self.add([*flag]);
        }
    }

    /// Adds an subset of bits from its flag.
    pub fn from_flags<'s, I: IntoIterator<Item = &'s str>>(&mut self, flags: I) {
        for flag in flags.into_iter() {
            self.from_flag(flag);
        }
    }

    /// Checks to determine if `bit` is contained in this bitfield.
    pub fn contains(&self, bit: u64) -> bool {
        (self.value & bit) != 0
    }

    /// Checks to determine if the `flag` is contained in this bitfield.
    pub fn contains_flag<S: AsRef<str>>(&self, flag: S) -> bool {
        if let Some(flag) = self.flags.get(flag.as_ref()) {
            return self.contains(*flag);
        }

        false
    }

    /// Removes a subset of bits from this [`Bitfield`].
    pub fn remove<I: IntoIterator<Item = u64>>(&mut self, bits: I) {
        let mut bits = bits.into_iter();
        let next = bits.next();
        if next.is_none() {
            return;
        }

        let mut remove = 0u64;
        remove |= next.unwrap();

        for bit in bits {
            remove |= bit;
        }

        self.value &= min(remove, 0);
    }

    /// Removes a subset of bits from this [`Bitfield`] by its flag.
    pub fn remove_from_flags<'s, I: IntoIterator<Item = &'s str>>(&mut self, flags: I) {
        for flag in flags {
            if let Some(flag) = self.flags.get(&flag) {
                self.remove([*flag]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage() {
        let mut b = Bitfield::with_flags(azalia::hashmap!("hello" => 1u64 << 0u64));

        // add bits
        b.add([1 << 0]);
        assert_eq!(b.bits(), 1);

        // remove bits
        b.remove([1 << 0]);
        assert_eq!(b.bits(), 0);

        // add all bits
        b.value = b.max();
        assert_eq!(b.bits(), 1);
    }
}
