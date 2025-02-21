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

mod apikeyscope;
mod member_permission;

pub use apikeyscope::*;
pub use member_permission::*;

use std::{cmp::min, collections::BTreeMap, marker::PhantomData};

/// Trait that implements the "scopes" concept.
///
/// The "scopes" concept is similar to [Discord's Permissions] where essentially
/// a **bitflags** represents a list of flags that will be serialized as a string
/// which uses bit-wise operations to determine if a flag is included.
///
/// [Discord's Permissions]: https://github.com/discord/discord-api-docs/blob/main/docs/topics/Permissions.md
pub trait Bitflags: Sized + Send + Sync {
    /// Type representation of a single bit.
    type Bit: Copy;

    /// Constant that represents a zero of the type representation
    /// of this trait.
    const ZERO: Self::Bit;

    /// Returns a [`BTreeMap`] of all possible flags avaliable.
    fn flags() -> BTreeMap<&'static str, Self::Bit>;

    /// Returns a slice of all avaliable bits from `0..{flags.len()}`.
    fn values<'v>() -> &'v [Self::Bit];

    fn max() -> Self::Bit
    where
        Self::Bit: Ord,
    {
        Self::values().iter().max().copied().unwrap_or(Self::ZERO)
    }
}

/// Data structure that easily do computations with `F::Bit` easily.
#[derive(Debug, Clone, Copy)]
pub struct Bitfield<F: Bitflags>(F::Bit, PhantomData<F>);
impl<F: Bitflags> Bitfield<F> {
    /// Create a new [`Bitfield`] data structure.
    pub const fn new(value: F::Bit) -> Bitfield<F> {
        Bitfield(value, PhantomData)
    }

    /// Returns the current bit value stored in this [`Bitfield`].
    pub const fn value(&self) -> F::Bit {
        self.0
    }
}

// Our implementation of bitflags (via `bitflags!`) use `u64` as the
// bit type repr. so it'll be tailoured to what we use for now.
//
// submit a pull request if you want other number types to be supported.
impl<F: Bitflags<Bit = u64>> Bitfield<F> {
    /// Returns all the possible enabled bits in the bitfield to determine
    pub fn flags(&self) -> Vec<(&'static str, F::Bit)> {
        F::flags().into_iter().filter(|(_, bit)| self.contains(*bit)).collect()
    }

    /// Adds multiple bits to this [`Bitfield`] and updating the current
    /// value to what was acculumated.
    ///
    /// ## Example
    /// ```rust
    /// # use charted_core::{bitflags, bitflags::Bitfield};
    /// #
    /// # bitflags! {
    /// #     #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    /// #     #[allow(clippy::enum_clike_unportable_variant)]
    /// #     #[repr(u64)]
    /// #     pub Scope[u64] {
    /// #         Hello["hello"] => 1u64 << 0u64;
    /// #         World["world"] => 1u64 << 1u64;
    /// #     }
    /// # }
    /// #
    /// let mut bitfield = Bitfield::<Scope>::new(0);
    /// bitfield.add([Scope::Hello]);
    /// assert_eq!(bitfield.value(), 1);
    /// ```
    //
    // I don't want to implement `Add` since I don't think doing:
    //
    //  let bitfield = Bitfield::<{some type}>::new();
    //  bitfield + 32;
    //
    // is a good API design choice.
    #[allow(clippy::should_implement_trait)]
    pub fn add<II: Into<F::Bit>, I: IntoIterator<Item = II>>(&mut self, values: I) {
        let iter = values.into_iter().map(Into::into);
        let new = iter.fold(self.0, |mut curr, elem: u64| {
            if elem == u64::MAX {
                return curr;
            }

            if elem > F::max() {
                return curr;
            }

            curr |= elem;
            curr
        });

        self.0 |= new
    }

    /// Removed multiple bits to this [`Bitfield`] and updating the current
    /// value to what was acculumated.
    ///
    /// ## Example
    /// ```
    /// # use charted_core::{bitflags, bitflags::Bitfield};
    /// #
    /// # bitflags! {
    /// #     #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    /// #     #[allow(clippy::enum_clike_unportable_variant)]
    /// #     #[repr(u64)]
    /// #     pub Scope[u64] {
    /// #         Hello["hello"] => 1u64 << 0u64;
    /// #         World["world"] => 1u64 << 1u64;
    /// #     }
    /// # }
    /// #
    /// let mut bitfield = Bitfield::<Scope>::new(0);
    ///
    /// bitfield.add([Scope::Hello]);
    /// assert_eq!(bitfield.value(), 1);
    ///
    /// bitfield.remove([Scope::Hello]);
    /// assert_eq!(bitfield.value(), 0);
    /// ```
    pub fn remove<II: Into<F::Bit>, I: IntoIterator<Item = II>>(&mut self, values: I) {
        let iter = values.into_iter().map(Into::into);
        let removed = iter.fold(self.0, |mut curr, elem: u64| {
            if elem == u64::MAX {
                return curr;
            }

            if elem > F::max() {
                return curr;
            }

            curr |= elem;
            curr
        });

        self.0 &= min(removed, 0)
    }

    /// Determines if `bit` is contained in the inner bit.
    pub fn contains<B: Into<F::Bit>>(&self, bit: B) -> bool {
        (self.value() & bit.into()) != 0
    }
}

impl<F: Bitflags> Default for Bitfield<F> {
    fn default() -> Self {
        Bitfield(F::ZERO, PhantomData)
    }
}

impl<F: Bitflags<Bit = u64>> FromIterator<u64> for Bitfield<F> {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut bitfield = Bitfield::<F>::default();
        bitfield.add(iter);

        bitfield
    }
}

/// Macro that create a enumeration to implement the [`Bitflags`] trait.
#[macro_export]
macro_rules! bitflags {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident[$bit:ty] {
            $(
                $(#[$doc:meta])*
                $field:ident[$key:literal] => $value:expr_2021;
            )*
        }
    ) => {
        $(#[$meta])*
        pub enum $name {
            $(
                $field = $value,
            )*
        }

        impl $name {
            pub const fn as_bit(self) -> $bit {
               self as u64
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.as_bit())
            }
        }

        impl ::core::convert::From<$name> for $bit {
            fn from(value: $name) -> $bit {
                value as $bit
            }
        }

        impl $crate::bitflags::Bitflags for $name {
            type Bit = $bit;
            const ZERO: Self::Bit = 0;

            #[inline]
            fn flags() -> ::std::collections::BTreeMap<&'static str, u64> {
                let mut map = ::std::collections::BTreeMap::new();
                $(map.insert($key, $value);)*

                map
            }

            fn values<'v>() -> &'v [$bit] {
                &[
                    $($value),*
                ]
            }
        }

        impl ::core::cmp::PartialEq<$bit> for $name {
            fn eq(&self, other: &$bit) -> bool {
                (*self as $bit) == *other
            }
        }
    };
}
