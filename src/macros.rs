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

/// Generic macro to create a [`HashMap`] easily.
///
/// ## Matches
/// ### `hashmap!()`
/// This will create an empty hashmap.
///
/// ```
/// # use charted::hashmap;
/// # use std::collections::HashMap;
/// #
/// let map: HashMap<&str, &str> = hashmap!();
/// assert_eq!(map.len(), 0);
/// ```
///
/// ### `hashmap! { key => value }`
/// Creates a empty [`HashMap`] and insert one or N elements in the HashMap.
///
/// ```
/// # use charted::hashmap;
/// #
/// let map = hashmap! {
///     "hello" => "world",
///     "world" => "hello"
/// };
///
/// assert_eq!(map.len(), 2);
/// ```
///
/// [HashMap]: std::collections::HashMap
#[macro_export]
macro_rules! hashmap {
    () => { ::std::collections::HashMap::new() };
    ($K:ty, $V:ty) => { ::std::collections::HashMap::<$K, $V>::new() };
    ($K:ty, $V:ty, { $($key:expr => $value:expr),* }) => {{
        let mut h = $crate::hashmap!($K, $V);
        $(
            h.insert($key, $value);
        )*

        h
    }};

    ($($key:expr => $value:expr),*) => {{
        let mut h = $crate::hashmap!();
        $(
            h.insert($key, $value);
        )*

        h
    }};
}

/// Macro to easily create a HashMap easily.
///
/// ## Example
/// ```
/// # use charted::hashset;
/// #
/// let map = hashset! {
///     "hello",
///     "world"
/// };
///
/// assert_eq!(map.len(), 2);
///
/// /*
/// expanded:
///
/// let map = {
///     let mut h = ::std::collections::HashSet::new();
///     h.insert("hello");
///     h.insert("world");
///
///     h
/// };
/// */
/// ```
#[macro_export]
macro_rules! hashset {
    () => { ::std::collections::HashSet::new() };
    ($V:ty) => { ::std::collections::HashSet::<$V>::new() };
    ($V:ty, $($value:expr),*) => {{
        let mut set = $crate::hashset!($V);
        $(
            set.insert($value);
        )*

        set
    }};

    ($($value:expr),*) => {{
        let mut h = $crate::hashset!();
        $(
            h.insert($value);
        )*

        h
    }};
}

/// General macro to implement `Lazy::new` from the `once_cell` library.
#[macro_export]
macro_rules! lazy {
    ($init:expr) => {{
        ::once_cell::sync::Lazy::new(|| $init)
    }};

    ($ty:ty) => {{
        ::once_cell::sync::Lazy::<$ty>::default()
    }};
}

/// General macro to create a [regular expression][regex::Regex] that you know
/// is valid.
#[macro_export]
macro_rules! regex {
    ($e:expr) => {
        ::regex::Regex::new($e).unwrap()
    };
}
