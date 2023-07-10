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

use std::{
    cmp,
    collections::{hash_map::Entry, HashMap, HashSet},
    hash::Hash,
    path::PathBuf,
};

use aws_sdk_s3::{
    config::Region,
    types::{BucketCannedAcl, ObjectCannedAcl},
};
use tracing::Level;

pub trait Merge {
    fn merge(&mut self, other: Self);
}

impl Merge for u16 {
    fn merge(&mut self, other: Self) {
        if cmp::Ord::cmp(self, &other) == cmp::Ordering::Less {
            *self = other;
        }
    }
}

impl Merge for u32 {
    fn merge(&mut self, other: Self) {
        if cmp::Ord::cmp(self, &other) == cmp::Ordering::Less {
            *self = other;
        }
    }
}

impl<T: Merge> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if self.is_none() {
            *self = other.take();
        }
    }
}

impl<T: Merge + Clone> Merge for Vec<T> {
    fn merge(&mut self, mut other: Self) {
        self.append(&mut other);
    }
}

impl Merge for String {
    fn merge(&mut self, other: Self) {
        if self.is_empty() {
            *self = other.clone();
            return;
        }

        if !other.is_empty() && *self != other {
            *self = other;
        }
    }
}

impl<'s> Merge for &'s str {
    fn merge(&mut self, other: Self) {
        if self.is_empty() {
            *self = other;
            return;
        }

        if !other.is_empty() && *self != other {
            *self = other;
        }
    }
}

impl<K: Eq + Hash, V: Merge> Merge for HashMap<K, V> {
    fn merge(&mut self, other: Self) {
        for (key, value) in other {
            match self.entry(key) {
                Entry::Occupied(mut e) => e.get_mut().merge(value),
                Entry::Vacant(empty) => {
                    empty.insert(value);
                }
            }
        }
    }
}

impl<T: Merge> Merge for HashSet<T> {
    fn merge(&mut self, other: Self) {
        *self = other;
    }
}

macro_rules! gen_equality_merges {
    ($($ty:ty, )*) => {
        $(
            impl Merge for $ty {
                fn merge(&mut self, other: Self) {
                    if *self == other {
                        return;
                    }

                    *self = other;
                }
            }
        )*
    };
}

gen_equality_merges!(bool, Level, ObjectCannedAcl, BucketCannedAcl, PathBuf, Region,);

#[cfg(test)]
mod tests {
    use crate::Merge;
    use charted_common::hashmap;

    #[test]
    fn merge_numbers() {
        let mut a = 0u32;
        let b = 1u32;

        a.merge(b);
        assert_eq!(a, 1);
    }

    #[test]
    fn merge_strings() {
        let mut a = "";
        let b = "awau";

        a.merge(b);
        assert_eq!(a, "awau");

        let mut a2 = String::from("awau");
        let b2 = String::new();
        a2.merge(b2);

        assert_eq!(a2.as_str(), "awau");
    }

    #[test]
    fn merge_vecs() {
        let mut my_vec = vec![1u32, 2u32, 3u32];
        my_vec.merge(vec![4u32, 5u32]);

        assert_eq!(my_vec.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge_option() {
        let mut some_option: Option<String> = None;
        some_option.merge(Some(String::from("woof")));

        assert!(some_option.is_some());
        assert_eq!(some_option.unwrap().as_str(), "woof");

        let mut other_option = Some("heck");
        other_option.merge(None);

        assert!(other_option.is_some());
        assert_eq!(other_option.unwrap(), "heck");
    }

    #[test]
    fn merge_maps() {
        let mut map = hashmap!("hello" => 0u32, "world" => 1u32);
        map.merge(hashmap!("woof" => 2u32, "wag" => 3u32));

        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&"woof"), Some(2u32).as_ref());
    }

    #[test]
    fn merge_bools() {
        let mut enabled = false;
        enabled.merge(true);

        assert!(enabled);

        let mut h = true;
        h.merge(false);

        assert!(!h);

        let mut same = true;
        same.merge(true);

        assert!(same);
    }
}
