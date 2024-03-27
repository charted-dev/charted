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

#[macro_export]
macro_rules! gen_rbac_enum {
    (
        $(#[$outer:meta])*
        $vis:vis $name:ident {
            $(
                $(#[$doc:meta])*
                $key:ident[$s:literal]: $value:expr;
            )*
        }
    ) => {
        $(#[$outer])*
        pub enum $name {
            $(
                $(#[$doc])*
                $key = $value,
            )*
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $name::$key => write!(f, "{}::{} ({})", stringify!($name), stringify!($key), $s),
                    )*
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        $name::$key => write!(f, "{} ({})", $s, $value),
                    )*
                }
            }
        }

        impl ::std::convert::From<$name> for u64 {
            fn from(value: $name) -> u64 {
                value as u64
            }
        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S: ::serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u64((*self).into())
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;
                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "a string or a number value in range of 1..{}", u64::MAX)
                    }

                    fn visit_u64<E: ::serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        if value >= u64::MAX {
                            return Err(::serde::de::Error::custom("value was greater or equal to `u64::MAX`"));
                        }

                        let max = $name::max();
                        if value > max {
                            return Err(::serde::de::Error::custom(format!("value is greater than maximum value acceptable ({max})")));
                        }

                        let map = $name::as_map();
                        match map.values().find(|x| ((**x) as u64) == value) {
                            Some(element) => Ok(*element),
                            None => Err(::serde::de::Error::custom(format!("unable to find value from number [{value}]")))
                        }
                    }

                    fn visit_str<E: ::serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        let map = $name::as_map();
                        match map.get(value) {
                            Some(element) => Ok(*element),
                            None => Err(::serde::de::Error::custom(format!("value [{value}] was not found")))
                        }
                    }
                }

                deserializer.deserialize_any(Visitor)
            }
        }

        impl $name {
            #[inline(always)]
            pub const fn as_str(&self) -> &str {
                match self {
                    $($name::$key => $s,)*
                }
            }

            #[inline(always)]
            pub fn values<'v>() -> &'v [&'v u64] {
                &[
                    $(&$value,)*
                ]
            }

            pub fn values_str<'v>() -> &'v [&'v str] {
                &[
                    $($s,)*
                ]
            }

            #[inline(always)]
            pub fn max() -> u64 {
                let elems = $name::values();
                **elems.iter().max().unwrap_or_else(|| &&0)
            }

            #[inline]
            pub fn as_map<'a>() -> ::std::collections::HashMap<&'a str, $name> {
                let mut h = ::std::collections::HashMap::<&'a str, $name>::new();
                $(h.insert($s, $name::$key);)*

                h
            }
        }
    };
}
