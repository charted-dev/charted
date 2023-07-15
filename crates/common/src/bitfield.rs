// 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Rust
// Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

use crate::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BitfieldError {
    #[error("You can't use u64::MAX to add or remove bits!")]
    MaxProvided,

    #[error("Bit overflow!!")]
    BitOverflow,
}

/// Represents a field of bits that is used to control RBAC permissions.
#[derive(Debug, Clone)]
pub struct Bitfield<'a> {
    flags: HashMap<&'a str, u64>,
    bits: u64,
}

impl<'a> Default for Bitfield<'a> {
    fn default() -> Bitfield<'a> {
        Bitfield::new(0, hashmap!())
    }
}

impl<'a> Bitfield<'a> {
    /// Creates a new [Bitfield] instance with the given bits
    /// and flags.
    pub fn new(bits: u64, flags: HashMap<&'a str, u64>) -> Bitfield {
        Bitfield { bits, flags }
    }

    /// Initialize a 0-bit field with specified flags.
    pub fn with_flags(flags: HashMap<&'a str, u64>) -> Bitfield<'a> {
        Bitfield { bits: 0, flags }
    }

    /// Initializes a new [Bitfield] with the given bits with the same
    /// flags, cloned.
    pub fn init(&self, bits: u64) -> Bitfield<'a> {
        Bitfield {
            bits,
            flags: self.flags.clone(),
        }
    }

    /// Returns a borrowed reference of the initialized flags
    /// by this bitfield.
    pub fn flags(&self) -> &HashMap<&'a str, u64> {
        &self.flags
    }

    /// Returns a copied value of the bits that are used in this [Bitfield].
    pub fn bits(&self) -> u64 {
        self.bits
    }

    /// Returns the maximum amount of bits from all the flags' values.
    pub fn max_bits(&self) -> u64 {
        self.flags.clone().into_values().fold(0, |acc, curr| acc | curr)
    }

    /// Adds all of the bits that were registered by the flags into
    /// the bitfield.
    pub fn add_all(&mut self) {
        let mut total = 0u64;
        for val in self.flags.clone().into_values() {
            total |= val;
        }

        self.bits |= total;
    }

    /// Adds multiple bits into one bit in this bitfield.
    pub fn add(&mut self, bits: &[u64]) -> Result<(), BitfieldError> {
        // Don't do anything if it is empty
        if bits.is_empty() {
            return Ok(());
        }

        let mut add_up_to = 0u64;
        for bit in bits.iter() {
            if *bit > self.max_bits() {
                return Err(BitfieldError::BitOverflow);
            }

            if *bit == u64::MAX {
                return Err(BitfieldError::MaxProvided);
            }

            add_up_to |= *bit;
        }

        self.bits |= add_up_to;
        Ok(())
    }

    /// Adds multiple bits by referencing flags instead of arbitary bits.
    pub fn add_from_flags(&mut self, bits: &[String]) -> Result<(), BitfieldError> {
        self.add(
            bits.iter()
                .map(|x| self.flags.get(&*x.to_string()))
                .filter(Option::is_some)
                .map(|x| *x.unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }

    /// Checks if the given flag was a valid flag and if the bit
    /// itself by the flag is contained in the bitfield or not.
    pub fn contains_flag<I: AsRef<str>>(&mut self, flag: I) -> bool {
        match self.flags.get(flag.as_ref()) {
            Some(bit) => (self.bits & *bit) != 0,
            None => false,
        }
    }

    /// Removes a subset of bits from the bitfield itself. This will
    /// use [std::cmp::min] if it flows into the negatives.
    pub fn remove(&mut self, bits: &[u64]) {
        let mut to_remove = 0u64;
        for bit in bits.iter() {
            to_remove |= *bit;
        }

        self.bits &= std::cmp::min(to_remove, 0);
    }

    /// Removes a subset of bits from the bitfield via the flags specified.
    pub fn remove_from_flags(&mut self, bits: &[String]) {
        self.remove(
            bits.iter()
                .map(|x| self.flags.get(&*x.to_string()))
                .filter(Option::is_some)
                .map(|x| *x.unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashmap;

    #[test]
    fn bitfield_usage() {
        let mut b = Bitfield::new(0, hashmap!("hello" => 1 << 0));

        // add bits
        assert!(b.add(&[1 << 0]).is_ok());
        assert_eq!(b.bits(), 1);

        let res = b.add_from_flags(&["world".into()]);
        assert!(res.is_ok());
        assert_eq!(b.bits(), 1);

        // remove bits
        b.remove(&[1 << 0]);
        assert_eq!(b.bits(), 0);
    }
}
