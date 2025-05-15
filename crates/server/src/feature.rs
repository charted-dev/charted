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
//
//! Allows building server features.

#![allow(unused)]

use charted_feature::Feature;
use std::{
    any::{self, TypeId},
    collections::{HashMap, hash_map::Iter},
    fmt::Debug,
    sync::Arc,
};

/// Newtype wrapper for holding a collection of [`Feature`]s.
#[derive(Clone, Default)]
pub struct Collection(HashMap<TypeId, Arc<dyn Feature>>);

impl Debug for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map = self
            .0
            .values()
            .map(|x| any::type_name_of_val(x.as_ref()))
            .collect::<Vec<_>>();

        f.debug_tuple("Features").field(&map).finish()
    }
}

impl Collection {
    /// Builds a new, empty feature collection.
    pub fn new() -> Self {
        Collection(HashMap::new())
    }

    /// Returns `true` if the feature is enabled.
    pub fn has<T: Feature + 'static>(&self) -> bool {
        // cloning the features has a minimal performance hit since it just increments
        // the reference count (and when dropped, decrements).
        self.0.values().cloned().any(|feat| feat.as_arc_any().is::<T>())
    }

    /// Returns a reference to the feature if it is enabled, otherwise [`None`] is
    /// returned.
    pub fn get<T: Feature>(&self) -> Option<&T> {
        if self.has::<T>() {
            // Since we know it exists (since `dyn Type::is` will ensure if
            // the type's ID of `self` is == type ID of `T`), we use `unwrap_unchecked()`
            // to ensure that it exists.
            let type_id = TypeId::of::<T>();

            // paranoia once more.
            assert!(self.0.contains_key(&type_id));

            // We shouldn't have a panic path here if we already know it exists.
            let feature = unsafe { self.0.get(&type_id).unwrap_unchecked() };

            // another paranoia check.
            debug_assert!(feature.is::<T>());
            return feature.downcast();
        }

        None
    }

    /// Returns a iterator of all the features avaliable
    pub fn iter(&self) -> Iter<'_, TypeId, Arc<dyn Feature>> {
        self.0.iter()
    }

    pub(crate) fn add<F: Feature>(&mut self, feat: F) {
        let type_id = TypeId::of::<F>();
        self.0.insert(type_id, Arc::new(feat));
    }
}

#[cfg(test)]
mod tests {
    use crate::feature::Collection;
    use charted_feature::{Feature, Metadata};

    struct AFeature;
    impl Feature for AFeature {
        fn metadata(&self) -> Metadata {
            const METADATA: Metadata = Metadata {
                name: "AFeature",
                config_key: "<none>",
                description: "a test feature, why are you looking at this?",
                authors: &["Noel Towa <cutie@floofy.dev>"],
                since: "0.0.0-devel.0",
                deprecated: None,
            };

            METADATA
        }
    }

    #[test]
    fn collection() {
        let mut features = Collection::new();

        features.add(AFeature);
        assert!(features.has::<AFeature>());

        let Some(x) = features.get::<AFeature>() else {
            panic!("failed to get `AFeature`");
        };

        assert_eq!(x.metadata(), Metadata {
            name: "AFeature",
            config_key: "<none>",
            description: "a test feature, why are you looking at this?",
            authors: &["Noel Towa <cutie@floofy.dev>"],
            since: "0.0.0-devel.0",
            deprecated: None,
        });
    }
}
