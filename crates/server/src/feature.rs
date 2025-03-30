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

use crate::Context;
use axum::Router;
use azalia::rust::AsArcAny;
use charted_core::BoxedFuture;
use std::{any::TypeId, collections::HashMap, sync::Arc};
use utoipa::openapi::OpenApi;

pub type Collection = HashMap<TypeId, Arc<dyn Feature>>;

/// Implements a **server feature**. Read in the [`features/` folder] for documentation.
///
/// [`features/` folder]: https://github.com/charted-dev/charted/blob/main/features/README.md
pub trait Feature: AsArcAny + Send + Sync + 'static {
    /// Does pre-initialization of this feature once the server is booting up.
    fn init<'feat, 'cx: 'feat>(&self, cx: &'cx Context) -> BoxedFuture<'feat, eyre::Result<()>> {
        let _ = cx;
        Box::pin(async { Ok(()) })
    }

    /// If the server feature implements new routes or data types, then this is where
    /// you can extend the core OpenAPI document.
    fn extends_openapi(&self, doc: &mut OpenApi) {
        let _ = doc;
    }

    /// Extends the API router.
    ///
    /// It returns a tuple of `(path, router)`. The `path` must be a valid path.
    fn extend_router(&self) -> (&'static str, Router<Context>);
}

azalia::impl_dyn_any!(Feature);
