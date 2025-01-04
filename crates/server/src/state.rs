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

use axum::extract::FromRef;
use charted_features::Feature;
use std::sync::{Arc, OnceLock};

static INSTANCE: OnceLock<ServerContext> = OnceLock::new();

/// Represents the context of the API server.
///
/// It extends the [`charted_app::Context`] object which also holds the
/// list of features that extend the functionality of the `charted_server`
/// crate.
#[derive(Clone, derive_more::Deref)]
pub struct ServerContext {
    #[deref]
    inner: charted_app::Context,

    pub features: Vec<Arc<dyn Feature>>,
}

impl ServerContext {
    pub(crate) fn new(cx: charted_app::Context, features: Vec<Arc<dyn Feature>>) -> ServerContext {
        ServerContext { inner: cx, features }
    }
}

impl ServerContext {
    /// Return a reference to the global [`ServerContext`] that was set from [`set_global`]. If
    /// [`set_global`] isn't called, then this will panic.
    pub fn get<'ctx>() -> &'ctx ServerContext {
        INSTANCE.get().expect("global server context was never initialized")
    }
}

impl FromRef<()> for ServerContext {
    fn from_ref(_: &()) -> Self {
        INSTANCE
            .get()
            .cloned()
            .expect("global server context was never initialized")
    }
}

pub(crate) fn set_global(ctx: ServerContext) {
    if INSTANCE.set(ctx).is_err() {
        panic!("a global server context has been set already");
    }
}
